#!/usr/bin/env scriptisto
// Copyright 2019 The Scriptisto Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// scriptisto-begin
// script_src: src/main/java/script/Script.java
// build_cmd: gradle build && tar xf ./build/distributions/java.java.tar --strip 1 -C .
// target_bin: ./bin/java.java
//
// files:
//  - path: build.gradle
//    content: |
//      apply plugin: 'java'
//      apply plugin: 'application'
//      mainClassName = 'script.Script'
//      tasks.distZip.enabled = false
//      repositories {
//        mavenCentral()
//      }
//      dependencies {
//        compile 'ch.qos.logback:logback-classic:1.2.3'
//      }
// scriptisto-end

package script;

import java.util.Date;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class Script{
	
	private static final Logger logger = LoggerFactory.getLogger(Script.class);
	
	public static void main(String[] args) {
		logger.debug("Hello, Java! Current Date : {}", new Date());
	}
}
