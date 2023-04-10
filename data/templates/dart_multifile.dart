#!/usr/bin/env scriptisto
# This template is meant to compile whole Dart projects instead of a single file
#
# scriptisto-begin
# script_src: dart_multifile_project/main.dart
# build_in_script_dir: true
# build_cmd: set -eu && dart compile exe "dart_multifile_project/main.dart" -o "${SCRIPTISTO_CACHE_DIR}/my-dart-program"
# target_bin: my-dart-program
# extra_src_paths:
#   - dart_multifile_project
# scriptisto-end
