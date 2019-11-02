#!/usr/bin/env scriptisto

# scriptisto-begin
# script_src: script.py
# build_cmd: mypy script.py && python3 -m compileall .
# target_interpreter: /usr/bin/env python3
# target_bin: ./script.py
# scriptisto-end

import argparse
from typing import Optional

def print_hello(input: Optional[int]):
    msg = "Hello, Python! Input: %d" % (input or 0)
    print(msg)

def main():
  parser = argparse.ArgumentParser()
  parser.add_argument("--input", type=int, help="Example input.")
  args = parser.parse_args()

  print_hello(args.input)

if __name__== "__main__":
  main()
