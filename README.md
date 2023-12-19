# minisort: The mini version of sort

This application is a miniature version of sort, the Unix-based CLI tool for sorting word lists.

## Requirements

To run from source, a Rust compiler (`rustc`) or the Rust package manager (`cargo`) is required. Alternatively, the binary `minisort` can be executed directly from the terminal.

## Usage

The following is the help text of the application. To display this text in a terminal, do either of the following:

1. In a directory containing the executable, run `minisort -h`
2. In the root directory of the project, run `cargo run -- -h`

```console
Usage: minisort [OPTIONS] <input> <output>

Arguments:
  <input>   Path to the file containing words to be sorted
  <output>  Path to the file to write sorted output to

Options:
  -a, --algorithm <algorithm>  Sorting algorithm to use [default: mergesort] [possible values: mergesort, quicksort, randomsort]
  -u, --unique                 Keep only unique words in output
  -c, --case_insensitive       Sort words case-insensitively (converts all words to lower-case)
  -h, --help                   Print help
  -V, --version                Print version
```

An example usage from the root directory of the project:

```console
cargo run -- input/test_file.txt output/output.txt -a quicksort -c -u
```

or using the executable:

```console
minisort input/test_file.txt output/output.txt -a quicksort -c -u
```

This command prompts the application to take the words listed in `input/test_file.txt`, sort them case-insensitively (`-c`) using the quicksort algorithm (`-a quicksort`), and stores the sorted deduplicated (`-u`) result in `output/output.txt`.

## Development

The application provides implementations of mergesort, quicksort, and random sort. Implementations for heapsort, Radix sort, and counting sort are still pending.
