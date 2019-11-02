#!/usr/bin/env scriptisto

-- scriptisto-begin
-- script_src: script.hs
-- build_cmd: ghc -O -o script script.hs && strip ./script
-- scriptisto-end

main = putStrLn "Hello, Haskell!"
