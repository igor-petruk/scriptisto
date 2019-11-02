#!/usr/bin/env scriptisto

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
