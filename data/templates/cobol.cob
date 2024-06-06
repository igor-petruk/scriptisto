#!/usr/bin/env scriptisto
      * scriptisto-begin
      * script_src: cobol.cob
      * build_cmd: cobc -x -o script ./cobol.cob
      * replace_shebang_with: '      * '
      * scriptisto-end
       IDENTIFICATION DIVISION.
       PROGRAM-ID. hello.
       PROCEDURE DIVISION.
       DISPLAY "Hello, COBOL!".
       STOP RUN.

