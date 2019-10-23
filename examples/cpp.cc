#!/usr/bin/env scriptisto
// Copyright 2019 The Scriptisto Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


#include <glibmm.h>
#include <iostream>

// scriptisto-begin
// script_src: main.cc
// build_cmd: clang++ -O2 main.cc `pkg-config --libs --cflags glibmm-2.4` -o ./app
// target_bin: "@@@/app"
// scriptisto-end

int main(int argc, char *argv[]) {
  const auto user = Glib::getenv("USER");
  std::cout << "Hello, C++! Current user: " << user << std::endl;
  return 0;
}
