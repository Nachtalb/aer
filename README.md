# aer

Simple web interface to search through tagged files. File tagging is done with
the filename: `(my_tag)(another tag).bin` Aliases are also managed when renaming
a file. So eg. the tag `shocked` becomes `(shock)(shocked)(surprised)(!)(😮)`.

While not implemented yet that's at least the goal. Basically a replacement of
[eary](https://github.com/Nachtalb/eary) written in rust to be faster.

## Run

```sh
# requisites
cargo install cargo-watch
# code analysis
rustup component add clippy
```

```sh
# Run
cargo watch -x run -- --path "/path/to/folder"
# Test
cargo watch -x test
# Code analysis
cargo watch -x clippy

# File serve with nginx
docker compose up -d
```

## License

[LGPL 3.0](LICENSE)
