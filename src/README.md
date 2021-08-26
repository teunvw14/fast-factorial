# fast-factorial | a multithreaded factorial calculator in Rust

What is says on the tin.

# Build

After cloning this repository, you should be able to build immediately using cargo:

```
$ cargo build --release
```

# Usage:

```
$ fast-factorial [number]
```

Example usage and output:
```
$ fast-factorial 100
Calculating 10! with 4 threads...
10! = 3628800
Done, took 0ms.
```