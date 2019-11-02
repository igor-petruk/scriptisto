#!/usr/bin/env scriptisto

// scriptisto-begin
// script_src: script.js
// build_once_cmd: npm install
// target_bin: ./script.js
// target_interpreter: /usr/bin/env node
// files:
//  - path: package.json
//    content: |
//     { "dependencies": { "yargs": "^14.2.0" } }
// scriptisto-end

argv = require('yargs')
    .usage('Usage: $0 [options]')
    .command('script', 'Example script')
    .alias('i', 'input').nargs('i', 1).describe('i', 'Example input')
    .help('h').alias('h', 'help')
    .argv;

console.log("Hello, JavaScript on Node!. Input: " + (argv.input || "<not set>"));
