# archivelfs

Allows retrieving [Git LFS](https://git-lfs.com/) files when [git-archive](https://git-scm.com/docs/git-archive) only returns pointers.

See <https://github.com/git-lfs/git-lfs/issues/1322>

### Usage

```
Allows retrieving Git LFS files when git-archive only provides pointers

Usage: archivelfs [OPTIONS] --lfs-url <LFS_URL> [ROOT]

Arguments:
  [ROOT]

Options:
      --jobs <JOBS>        [default: 4]
  -h, --help               Print help
```

### Installation

```
$ cargo install --git https://github.com/cpg314/archivelfs
```
