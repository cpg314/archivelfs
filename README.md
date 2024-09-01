# archivelfs

Allows retrieving [Git LFS](https://git-lfs.com/) files when [git-archive](https://git-scm.com/docs/git-archive) only returns pointers.

See <https://github.com/git-lfs/git-lfs/issues/1322>

## Usage

```
Allows retrieving Git LFS files when git-archive only provides pointers
Usage: archivelfs [OPTIONS] [ROOT]

Arguments:
  [ROOT]  Repository root

Options:
      --jobs <JOBS>  [default: 4]
  -h, --help         Print help
  -V, --version      Print version
```

## Installation

### Binaries

The [releases page](https://github.com/cpg314/archivelfs/releases) contains packages for Ubuntu/Debian, Arch Linux, as well as tarballs.

### From source

```
$ cargo install --git https://github.com/cpg314/archivelfs
```
