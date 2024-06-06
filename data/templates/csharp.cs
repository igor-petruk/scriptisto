#!/usr/bin/env scriptisto

// scriptisto-begin
// script_src: Program.cs
// target_bin: bin/Release/net8.0/script
// build_cmd: dotnet build -c Release script.csproj
// files:
//  - path: script.csproj
//    content: |
//      <Project Sdk="Microsoft.NET.Sdk">
//        <PropertyGroup>
//          <OutputType>Exe</OutputType>
//          <TargetFramework>net8.0</TargetFramework>
//          <ImplicitUsings>enable</ImplicitUsings>
//          <Nullable>enable</Nullable>
//        </PropertyGroup>
//      </Project>
// scriptisto-end

// See https://aka.ms/new-console-template for more information

var name = Environment.GetEnvironmentVariable("USER");
Console.BackgroundColor = ConsoleColor.DarkBlue;
Console.ForegroundColor = ConsoleColor.White;
Console.Write("Hello");
Console.ResetColor();
Console.WriteLine($", {name}!");
