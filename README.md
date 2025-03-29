# csv-compare

csv-compare is a cli difftool for csv files.

[![Build status](https://github.com/attilarepka/csv-compare/actions/workflows/tests.yml/badge.svg)](https://github.com/attilarepka/csv-compare/actions)

## Features

- `git diff` like diffing of csv files
- filter column prefixes for specific text

## Installation

**[Archives of precompiled binaries for csv-compare are available for 
macOS and Linux.](https://github.com/attilarepka/csv-compare/releases)**

Linux binaries are static executables.

If you're a **Debian** user (or a user of a Debian derivative like **Ubuntu**),
then csv-compare can be installed using a binary `.deb` file provided in each
[csv-compare release](https://github.com/attilarepka/csv-compare/releases).

```
$ curl -LO https://github.com/attilarepka/csv-compare/releases/download/0.1.0/csv-compare_0.1.0_amd64.deb
$ sudo dpkg -i csv-compare_0.1.0_amd64.deb
```

### Building

csv-compare is written in Rust, so you'll need [Rust installation](https://www.rust-lang.org/) in order to compile it.
csv-compare compiles with Rust 1.70.0 (stable) or newer. In general, it tracks
the latest stable release of the Rust compiler.

```shell
$ git clone https://github.com/attilarepka/csv-compare.git
$ cd csv-compare
$ cargo build --release
```

## Usage

Expurgator provides a command-line interface with the following options:

```shell
Usage: csv-compare [OPTIONS] --orig-index <ORIG_INDEX> <ORIG> <DIFF>

Arguments:
  <ORIG>  Orig CSV file
  <DIFF>  Diff CSV file

Options:
      --orig-index <ORIG_INDEX>    Orig index of column to compare
      --diff-index <DIFF_INDEX>    Diff index of column to compare (optional, defaults to orig_index)
  -w, --with-prefix <WITH_PREFIX>  Search prefix of selected rows
      --with-headers               Whether CSV's have headers
  -h, --help                       Print help
  -V, --version                    Print version
```

## Contributing

Contributions are welcome! Open a GitHub issue or pull request.

## License

This project is licensed under the [MIT license](LICENSE)

