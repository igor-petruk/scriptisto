#!/usr/bin/env scriptisto
;; Copyright 2019 The Scriptisto Authors
;;
;; Licensed under the Apache License, Version 2.0 (the "License");
;; you may not use this file except in compliance with the License.
;; You may obtain a copy of the License at
;;
;;      http://www.apache.org/licenses/LICENSE-2.0
;;
;; Unless required by applicable law or agreed to in writing, software
;; distributed under the License is distributed on an "AS IS" BASIS,
;; WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
;; See the License for the specific language governing permissions and
;; limitations under the License.



;     scriptisto-begin
;     script_src: script.lisp
;     build_cmd: chmod +x build.lisp && ./build.lisp
;     target_bin: @@@/script
;
;     file-begin: build.lisp
;     #!/usr/bin/sbcl --script
;     (load "script.lisp")
;     (sb-ext:save-lisp-and-die "script"
;                           :executable t
;                           :toplevel 'main)
;     file-end: build.lisp
;
;     scriptisto-end

(defun main ()
      (format t "Hello, Common Lisp!~%"))

