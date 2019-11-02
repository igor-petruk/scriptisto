#!/usr/bin/env scriptisto

package main

// scriptisto-begin
// script_src: main.go
// build_once_cmd: go get github.com/fatih/color
// build_cmd: go build -o script
// replace_shebang_with: //
// scriptisto-end

import "github.com/fatih/color"

func main() {
	color.Yellow("Hello, Go!")
}
