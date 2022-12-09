#!/usr/bin/env scriptisto
/*
 scriptisto-begin
 script_src: cli/main.dart
 build_cmd: dart compile exe "cli/main.dart" -o my-dart-program
 target_bin: ./my-dart-program
 scriptisto-end
*/

void main(List<String> arguments) { 
  print('Hello Dart, arguments=' + arguments.toString()); 
}
