#!/bin/env scriptisto

# scriptisto-begin
# script_src: script.cr
# build_cmd: shards build --production && strip ./bin/script
# target_bin: @@@/bin/script
# file-begin: shard.yml
# name: script
# version: 0.1.0

# targets:
#   script:
#     main: script.cr
# file-end: shard.yml
# scriptisto-end

puts "Hello, Crystal!"
