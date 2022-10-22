# `bump`

A SemVer version bumper

## Installation

```shell script
cargo install --git https://github.com/broothie/bump
```

## Usage

### Example

```console
$ cat version.rb
VERSION = 'v3.2.1'.freeze

$ # Bump patch version
$ bump version.rb
3.2.1 -> 3.2.2
$ cat version.rb
VERSION = 'v3.2.2'.freeze

$ # Bump minor version
$ bump version.rb -s minor
3.2.2 -> 3.3.0
$ cat version.rb
VERSION = 'v3.3.0'.freeze

$ # Bump major version
$ bump version.rb -s major
3.3.0 -> 4.0.0
$ cat version.rb
VERSION = 'v4.0.0'.freeze
```

### Options

```console
$ bump -h
A SemVer version bumper

Usage: bump [OPTIONS] <FILENAME>

Arguments:
  <FILENAME>

Options:
  -s, --segment <SEGMENT>  SemVer segment to bump [default: patch] [possible values: patch, minor, major]
  -n, --number <NUMBER>    If the input contains multiple SemVer patterns, use this to target by occurrence (1-indexed) [default: 1]
  -l, --line <LINE>        If the input contains multiple SemVer patterns, use this to target by line number (1-indexed)
  -h, --help               Print help information
  -V, --version            Print version information
```
