# minecraft

Run a headless Minecraft server.

## Usage
- `java -jar path/to/server.jar --nogui` : start a Minecraft server and generate a world if it doesn't exist
- `java -Xms1024M -Xmx2048M -jar path/to/server.jar --nogui` : set the minimum and maximum amount of memory a server is allowed to have (Note: Setting them the same prevents lag caused by heap scaling)
- `java -jar path/to/server.jar` : start a server with a GUI
- `stop` : [Interactive] Shut the server down

## Categories
files

## Links
- More information: https://minecraft.wiki/w/Tutorial:Setting_up_a_Java_Edition_server
- tldr page: https://tldr.inbrowser.app/pages/common/minecraft
