#!/bin/env scriptisto
# Copyright 2019 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.


# scriptisto-begin
# script_src: script.py
# build_cmd: mypy script.py && python3 -m compileall .
# target_bin: /usr/bin/python3 @@@/script.py
# scriptisto-end

def print_int(s: str):
    print("Hello, %s!" % s)

def main():
  print_int("Python")
  
if __name__== "__main__":
  main()