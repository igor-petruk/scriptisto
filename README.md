# Scriptisto

**This is not an officially supported Google product**

## Installation

Install Rust, for example via https://rustup.rs/. Then

```shell
$ cd scriptisto
$ cargo install --force --path .
```

## Running

Try one of the script templates from one of the language available. To see the
list:

```shell
$ scriptisto -g
```

Then generate your basic script:

```shell
$ scriptisto -g rust | tee ./rust-script
$ chmod +x ./rust-script
$ ./rust-script
Hello, Rust!
```

## Contributing

See [Contributing Guide](docs/contributing.md).
