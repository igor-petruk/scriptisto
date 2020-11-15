#!/usr/bin/env scriptisto
# This compiles "cli/main.dart" from the source directory and uses *this* script to run it.
#
# To get started:
# 1) mkdir -p cli && echo "void main(List<String> arguments) { print('Hello Dart, arguments=' + arguments.toString()); }" >cli/main.dart
# 2) scriptisto new dart >my-program
# 3) chmod +x my-program
# 4) ./my-program
#
# scriptisto-begin
# script_src: runner
# build_cmd: cache_dir="$(pwd)" && cd "$(dirname "${SCRIPTISTO_SOURCE}")" && dart2native "cli/main.dart" -o "${cache_dir}/my-dart-program"
# target_bin: ./runner
# target_interpreter: /bin/sh
# hash_additional_paths:
#   - cli # may contain more Dart source files
# scriptisto-end

# Since `SCRIPTISTO_CACHE_DIR` is known, more preparations can be done here, such as switching
# to an expected directory before running the program:
set -e
cd cli

exec "${SCRIPTISTO_CACHE_DIR}/my-dart-program" "${@}"
