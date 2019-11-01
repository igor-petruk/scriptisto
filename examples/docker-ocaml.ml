#!/usr/bin/env scriptisto

(*
  scriptisto-begin
  script_src: script.ml
  build_cmd: cd /src/ && opam exec -- dune build script.exe
  target_bin: ./_build/default/script.exe
  docker_build:
    dockerfile: |
      FROM ocaml/opam2:alpine
      RUN sudo apk add m4
      RUN opam install -y lwt
    src_mount_dir: /src
  files:
   - path: dune
     content: (executable (name script) (libraries lwt.unix))
   - path: dune-workspace
     content: |
       (lang dune 1.11)
       (env (_ (flags -cclib -static)))
  scriptisto-end
*)

Lwt_main.run (Lwt_io.printf "Hello, OCaml!\n")

