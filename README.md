# Scriptisto

[![Latest Version](https://img.shields.io/crates/v/scriptisto.svg)](https://crates.io/crates/scriptisto)
[![Build Status](https://cloud.drone.io/api/badges/igor-petruk/scriptisto/status.svg)](https://cloud.drone.io/igor-petruk/scriptisto)
![Crates.io License](https://img.shields.io/crates/l/scriptisto)
![Libraries.io dependency status for latest release](https://img.shields.io/librariesio/release/cargo/scriptisto)
![GitHub top language](https://img.shields.io/github/languages/top/igor-petruk/scriptisto)
![Crates.ioi Downloads](https://img.shields.io/crates/d/scriptisto)

It is tool to enable writing one file scripts in languages that require compilation, dependencies fetching or preprocessing.

It works as a "shebang" for those scripts, extracting build instructions from comments. If a script is changed, it rebuilds it and caches the result. If it was already built, it immediately delegates to a binary with only <1 ms overhead.

Also useful for non-compiled languages like Typed Python, but type validation is slow and a good idea to do it once after very change. More advantages are listed in [Wiki](https://github.com/igor-petruk/scriptisto/wiki#advantages).

## Demo

```c
#!/usr/bin/env scriptisto

#include <stdio.h>
#include <glib.h>

// scriptisto-begin
// script_src: main.c
// build_cmd: clang -O2 main.c `pkg-config --libs --cflags glib-2.0` -o ./script
// scriptisto-end

int main(int argc, char *argv[]) {
  gchar* user = g_getenv("USER");
  printf("Hello, C! Current user: %s\n", user);
  return 0;
}
```

```shell
$ chmod +x ./script.c
$ ./script.c
Hello, C! Current user: username
```

## Installation

Install Rust, for example via https://rustup.rs/. Then install from Crates.io:

```shell
$ cargo install scriptisto
```

Or the latest from Github:

```shell
$ git clone https://github.com/igor-petruk/scriptisto.git
$ cd scriptisto
$ cargo install --path .
```

## Documentation

Proceed to our [Wiki](https://github.com/igor-petruk/scriptisto/wiki).

## Disclaimer

This is not an officially supported Google product.
