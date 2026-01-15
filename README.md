# Hytale Plugin Initializer
A command line tool to initialize a template for your Hytale Plugin

## Usage:

### <div id="usage-0">Direct installation from Releases page</div>

- Download the latest `hytale_plugin_init.zip` file from the Releases tab
- Unzip it on your machine
- Run the `hytale_plugin_init.exe` file
- Select desired options
- Place the `HytaleServer.jar` file in the `resources/` directory
- Run the following command:
```bat
mvn install:install-file -Dfile="resources/HytaleServer.jar" -DpomFile="resources/hytale-plugin-init.pom"
```
- Open the output directory with intellij IDEA


### Building it yourself
- Install rust and cargo on your machine
- Clone this repo on your machine
- Run `cargo build --release` in the terminal
- Copy the `resources/` directory and the `target/release/hytale_plugin_init.exe` to another directory
- Continue from Step 3 of [Direct installation from Releases page](#usage-0)
