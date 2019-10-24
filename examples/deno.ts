#!/usr/bin/env scriptisto

// scriptisto-begin
// script_src: script.ts
// build_cmd: deno bundle script.ts
// target_bin: ./script.bundle.js
// target_interpreter: /usr/bin/env deno --no-prompt
// scriptisto-end

import { yellow, bold } from "https://deno.land/std/fmt/colors.ts";

console.log(yellow(bold("Hello, TypeScript on Deno!")));
