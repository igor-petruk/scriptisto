#!/usr/bin/env scriptisto

/*
  scriptisto-begin
  script_src: script.re
  build_cmd: dune build script.exe
  target_bin: ./_build/default/script.exe
  files:
   - path: dune
     content: (executable (name script) (libraries lwt.unix))
  scriptisto-end
*/

Lwt_main.run (Lwt_io.printf ("Hello, ReasonML!\n"));
