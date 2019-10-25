#!/usr/bin/env scriptisto

; scriptisto-begin
; script_src: ./script.rkt
; build_cmd: raco make script.rkt
; target_bin: ./compiled/script_rkt.zo
; target_interpreter: /usr/bin/env racket -t
; scriptisto-end

#lang racket/base
(displayln "Hello, Racket!")
