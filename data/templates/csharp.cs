#!/usr/bin/env scriptisto

using System;

// scriptisto-begin
// script_src: csharp.cs
// build_once_cmd: dotnet new console --name csharp --force
// # TODO: this works but could use refactoring
// build_cmd: "SCRIPT_NAME=csharp && echo current_directory: $(pwd) && cp $SCRIPT_NAME.cs $SCRIPT_NAME/Program.cs && cd $SCRIPT_NAME && dotnet build --configuration Release"
// # Change to dotnet version your system uses: net6.0, net7.0, net8.0
// target_bin: ./csharp/bin/Release/net8.0/csharp
// scriptisto-end

/*
# How build process works
Scriptisto Creates: $HOME/.cache/scriptisto/$(pwd)/${SCRIPT_NAME}/
Dotnet Creates new console project in current directory
Copy $SCRIPT_NAME.cs to CSHARP_PROJECT_DIR/Program.cs to preserve project directory structure
cd to CSHARP_PROJECT_DIR to find .csproj
Dotnet Build: the project
*/

public class Program {
    public static void Main(string[] args){
        Console.WriteLine("Scriptisto: Hello World!");
        Console.WriteLine($"args.Length: {args.Length}");
        for (int i=0; i<args.Length; i++){
            Console.WriteLine($"Arg[{i}] [{args[i]}]");
        }
    }
}
