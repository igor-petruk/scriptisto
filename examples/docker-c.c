#!/usr/bin/env scriptisto

#include <glib.h>
#include <stdio.h>

// scriptisto-begin
// script_src: main.c
// build_cmd: clang -static -O2 main.c `pkg-config --libs --cflags glib-2.0` -o ./script
// docker_build:
//    src_mount_dir: /src
//    dockerfile: |
//      FROM alpine
//      WORKDIR /src
//      RUN apk add glib-static glib-dev clang libc-dev build-base binutils pkgconfig
// scriptisto-end

int main(int argc, char* argv[]) {
  const gchar* user = g_getenv("USER");
  printf("Hello, C built in Docker! Current user: %s\n", user);
  return 0;
}
