#!/usr/bin/env scriptisto

# scriptisto-begin
# script_src: script.cr
# build_cmd: shards build --production && strip ./bin/script
# target_bin: ./bin/script
# files:
#  - path: shard.yml
#    content: |
#     name: script
#     version: 0.1.0
#     targets:
#       script:
#         main: script.cr
# scriptisto-end

puts "Hello, Crystal!"
