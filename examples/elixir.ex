#!/bin/env scriptisto

# Copyright 2019 The Scriptisto Authors
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
# script_src: lib/script.ex
# build_cmd: MIX_ENV=prod mix escript.build
# target_bin: @@@/script
#
# file-begin: mix.exs
# defmodule Script.MixProject do
#  use Mix.Project
#
#  def project do
#    [
#      app: :script,
#      version: "0.1.0",
#      elixir: "~> 1.8",
#      escript: [main_module: Script.CLI],
#    ]
#  end
# end
# file-end: mix.exs
#
# scriptisto-end

defmodule Script.CLI do
  def main(_) do 
    IO.puts "Hello, Elixir!"
  end
end
