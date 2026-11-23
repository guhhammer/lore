# mods

AI for the command-line, built for pipelines.

## Usage
- `mods "write me a poem about platypuses"` : ask a generic question
- `mods --settings` : open settings in your `$EDITOR`
- `mods <path/to/file -f "what are your thoughts on improving this code?"` : ask for comments on your code, in markdown format
- `mods <README.md -f "write a new section to this readme for a feature that sends you a free rabbit if you hit r"` : ask for help with your documentation, in markdown format
- `ls path/to/videos | mods -f "organize these by decade and summarize"` : organize your videos, in markdown format
- `curl "https://api.open-meteo.com/v1/forecast?latitude=29.00&longitude=-90.00&current_weather=true&hourly=temperature_2m,relativehumidity_2m,windspeed_10m" | mods -f "summarize this weather data for a human"` : read through raw HTML and summarize the contents, in markdown format
- `mods --help` : display help

## Categories
documents, files, network

## Links
- More information: https://github.com/charmbracelet/mods#usage
- tldr page: https://tldr.inbrowser.app/pages/common/mods
