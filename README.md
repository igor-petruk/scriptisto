# Scriptisto

**This is not an officially supported Google product**

## Demo

```c
#!/usr/bin/env scriptisto

#include <stdio.h>
#include <glib.h>

// scriptisto-begin
// script_src: main.c
// build_cmd: clang -O2 main.c `pkg-config --libs --cflags glib-2.0` -o ./app 
// target_bin: @@@/app
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
