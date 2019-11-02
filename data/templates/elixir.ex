#!/usr/bin/env scriptisto

# scriptisto-begin
# script_src: lib/script.ex
# build_cmd: MIX_ENV=prod mix escript.build
# files:
#  - path: mix.exs
#    content: |
#     defmodule Script.MixProject do
#     use Mix.Project
#     def project do
#      [
#        app: :script,
#        version: "0.1.0",
#        elixir: "~> 1.8",
#        escript: [main_module: Script.CLI],
#      ]
#      end
#     end
# scriptisto-end

defmodule Script.CLI do
  def main(_) do 
    IO.puts "Hello, Elixir!"
  end
end
