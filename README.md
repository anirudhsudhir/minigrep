# Minigrep

A clone of the popular command-line tool `grep`, written in Rust.

## Usage

```sh
cargo build --release
cp target/release/minigrep ./
```

./minigrep [query_string] [filepath] [flags...]

### Flags

- `-i` : Performs case-insensitive text matching

### Environment variables

- `IGNORE_CASE` : Performs case-insensitive text matching if set to any value

#### Notes

The application redirects errors to the standard error stream

Built following this [guide](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html)
