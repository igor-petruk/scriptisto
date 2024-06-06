#!/usr/bin/env scriptisto

package main

// scriptisto-begin
// script_src: main.go
// build_once_cmd: go mod tidy
// build_cmd: go build -o script
// replace_shebang_with: //
// files:
//  - path: go.mod
//    content: |
//      module example.com/a/b
//      require (
//          github.com/fatih/color v1.13.0
//      )
// scriptisto-end

import "github.com/fatih/color"

func main() {
	color.Yellow("Hello, Go!")
}
