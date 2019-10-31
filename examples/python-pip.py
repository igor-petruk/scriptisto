#!/usr/bin/env scriptisto

# scriptisto-begin
# script_src: script.py
# build_once_cmd: virtualenv -p python3 . && source ./bin/activate && pip install mypy termcolor
# build_cmd: source ./bin/activate && mypy script.py && python3 -m compileall . && chmod +x ./run.sh
# target_bin: ./run.sh
# files:
#   - path: run.sh
#     content: |
#       #!/bin/sh
#       export DIR=$(dirname $0)
#       source $DIR/bin/activate
#       python3 $DIR/script.py $@
# scriptisto-end

from termcolor import colored, cprint

def print_str(s: str):
    cprint("Hello, %s!" % s, 'green')

def main():
  print_str("Python")
  
if __name__== "__main__":
  main()
