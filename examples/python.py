#!/usr/bin/env scriptisto

# scriptisto-begin
# script_src: script.py
# build_cmd: mypy script.py && python3 -m compileall .
# target_interpreter: /usr/bin/env python3
# target_bin: ./script.py
# scriptisto-end

def print_str(s: str):
    print("Hello, %s!" % s)

def main():
  print_str("Python")
  
if __name__== "__main__":
  main()
