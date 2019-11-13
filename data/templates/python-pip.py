#!/usr/bin/env scriptisto

# scriptisto-begin
# script_src: script.py
# build_once_cmd: virtualenv -p python3 . && . ./bin/activate && pip install mypy termcolor
# build_cmd: . ./bin/activate && mypy script.py && python3 -m compileall . && chmod +x ./run.sh
# target_bin: ./run.sh
# files:
#   - path: run.sh
#     content: |
#       #!/bin/sh
#       export DIR=$(dirname $0)
#       . $DIR/bin/activate
#       python3 $DIR/script.py $@
# scriptisto-end

import argparse
from termcolor import colored, cprint
from typing import Optional

def print_hello(input: Optional[int]):
    msg = "Hello, Python! Input: %d" % (input or 0)
    cprint(msg, 'green')

def main():
  parser = argparse.ArgumentParser()
  parser.add_argument("--input", type=int, help="Example input.")
  args = parser.parse_args()

  print_hello(args.input)
  
if __name__== "__main__":
  main()
