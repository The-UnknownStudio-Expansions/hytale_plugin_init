@echo off

@REM ! edit the values inside double quotes ("") as your needs

hytale_plugin_init ^
	--output-dir "output" ^
	--group "com.killersnake" ^
	--name "MyBrandNewPlugin" ^
	--description "some epic plugin ready to change the world" ^
	--website "www.github.com/killersnake3" ^
	--author-name "killersnake3" ^
	--author-email "killersnake190@gmail.com" ^
	--author-url "www.github.com/killersnake3"



@REM ! Do not edit past this ! #

mvn install:install-file -Dfile="resources/HytaleServer.jar" -DpomFile="resources/hytale-plugin-init.pom"