# Scriptisto

[![Latest Version](https://img.shields.io/crates/v/scriptisto.svg)](https://crates.io/crates/scriptisto)
[![Build Status](https://cloud.drone.io/api/badges/igor-petruk/scriptisto/status.svg)](https://cloud.drone.io/igor-petruk/scriptisto)
![Crates.io License](https://img.shields.io/crates/l/scriptisto)
![Libraries.io dependency status for latest release](https://img.shields.io/librariesio/release/cargo/scriptisto)
![GitHub top language](https://img.shields.io/github/languages/top/igor-petruk/scriptisto)

![Crates.io](https://img.shields.io/crates/d/scriptisto?label=Cargo.io%20downloads)
![GitHub All Releases](https://img.shields.io/github/downloads/igor-petruk/scriptisto/total?logo=Github&label=Github%20Release%20downloads)

It is tool to enable writing one file scripts in languages that require compilation, dependencies fetching or preprocessing.

It works as a "shebang" for those scripts, extracting build instructions from comments. If a script is changed, it rebuilds it and caches the result. If it was already built, it immediately delegates to a binary with only <1 ms overhead.

Advantages and use-cases are listed in the [Wiki](https://github.com/igor-petruk/scriptisto/wiki#advantages).

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

Scriptisto is available as a prebuilt statically-linked standalone binary or distrubutions packages at [Releases](https://github.com/igor-petruk/scriptisto/releases) or at [Crates.io](https://crates.io/crates/scriptisto). 

Please proceed to the [Installation](https://github.com/igor-petruk/scriptisto/wiki/Installation) for instructions.

## Documentation

Proceed to our [Wiki](https://github.com/igor-petruk/scriptisto/wiki).

## Disclaimer

This is not an officially supported Google product.
