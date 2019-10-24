#!/usr/bin/env scriptisto

package main

// scriptisto-begin
// script_src: main.go
// build_cmd: go build -o script
// replace_shebang_with: //
// scriptisto-end

// Please run "go get" for this package.
import "github.com/fatih/color"

func main() {
	color.Yellow("Hello, Go!")
}
