#!/usr/bin/env scriptisto

; scriptisto-begin
; script_src: script.lisp
; build_cmd: chmod +x build.lisp && ./build.lisp
; files:
;  - path: build.lisp
;    content: |
;     #!/usr/bin/sbcl --script
;     (load "script.lisp")
;     (sb-ext:save-lisp-and-die "script"
;                     :executable t
;                     :toplevel 'main)
; scriptisto-end

(defun main ()
      (format t "Hello, Common Lisp!~%"))

