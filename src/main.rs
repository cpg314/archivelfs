use std::path::{Path, PathBuf};

use clap::Parser;
use futures::stream::{self, StreamExt, TryStreamExt};
use log::*;

#[derive(Parser)]
struct Flags {
    root: Option<PathBuf>,
    #[clap(long)]
    lfs_url: String,
    #[clap(long, default_value_t = 4)]
    jobs: usize,
}

#[derive(Debug)]
struct LfsPointer {
    sha256: String,
    size: usize,
    filename: PathBuf,
}
impl LfsPointer {
    fn parse(filename: &Path) -> anyhow::Result<Self> {
        let x = std::fs::read_to_string(filename)?;
        let lines: Vec<_> = x.lines().collect();
        anyhow::ensure!(lines.len() == 3);
        anyhow::ensure!(lines[0] == "version https://git-lfs.github.com/spec/v1");
        let Some(sha256) = lines[1].strip_prefix("oid sha256:") else {
            anyhow::bail!("Invalid oid: {:?}", lines[1]);
        };
        let Some(Ok(size)) = lines[2].strip_prefix("size ").map(|s| s.parse()) else {
            anyhow::bail!("Invalid size: {:?}", lines[2]);
        };
        Ok(Self {
            sha256: sha256.into(),
            size,
            filename: filename.into(),
        })
    }
}

async fn main_impl() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let mut args = Flags::parse();

    let root = if let Some(root) = args.root {
        root
    } else {
        std::env::current_dir()?
    };
    let files: Vec<_> = walkdir::WalkDir::new(root)
        .min_depth(1)
        .into_iter()
        // Exclude target/ for the root package
        .filter_entry(|e| !e.path().join("CACHEDIR.TAG").exists())
        .filter_map(|e| e.ok())
        .filter(|f| f.metadata().map_or(false, |m| m.len() < 200))
        .filter_map(|f| LfsPointer::parse(f.path()).ok())
        .collect();
    info!(
        "Found {} pointer files (total size {:.2} MB)",
        files.len(),
        files.iter().map(|f| f.size as f64 / 1e6).sum::<f64>()
    );
    let client = reqwest::Client::new();

    if !args.lfs_url.ends_with('/') {
        args.lfs_url += "/";
    }
    let url = reqwest::Url::parse(&args.lfs_url)?;
    let url = url.join("")?.join("object/")?;
    let start = std::time::Instant::now();
    info!("Downloading...");
    stream::iter(files)
        .map(|f| {
            let client = client.clone();
            let url = url.clone();
            async move {
                let url = url.join(&f.sha256)?;
                let data = client
                    .get(url)
                    .send()
                    .await?
                    .error_for_status()?
                    .bytes()
                    .await?;

                tokio::fs::write(f.filename, data).await?;
                anyhow::Ok(())
            }
        })
        .buffer_unordered(args.jobs)
        .try_collect::<Vec<_>>()
        .await?;
    info!("Done downloading in {:?}", start.elapsed());
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = main_impl().await {
        error!("{}", e);
        std::process::exit(2);
    }
}
