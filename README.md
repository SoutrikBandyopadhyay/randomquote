# randomquote

This is a binary application that generates a random quote from a corpus of JSON
data provided herein. The application can be used along with
[Conky](https://github.com/brndnmtthws/conky) desktop
manager to dynamically generate quotes.

## Usage

```shell
$ cargo build --release
$ cp /target/release/randomquote .
$ ./randomquote
```

## Benchmarking

The benchmarking was done with [hyperfine](https://github.com/sharkdp/hyperfine)
to compare against a Python3 implementation

```shell
hyperfine --runs 100 './randomquote' 'python3 quotes.py'
```

The repository author obtained a ~7.2 times speedup against python
implementation
