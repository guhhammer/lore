# lxi

Control LXI compatible instruments such as oscilloscopes.

## Usage
- `lxi discover` : discover LXI devices on available networks
- `lxi screenshot -a ip_address` : capture a screenshot, detecting a plugin automatically
- `lxi screenshot -a ip_address -p rigol-1000z` : capture a screenshot using a specified plugin
- `lxi scpi -a ip_address "*IDN?"` : send an SCPI command to an instrument
- `lxi benchmark -a ip_address` : run a benchmark for request and response performance

## Categories
media, monitoring

## Links
- More information: https://github.com/lxi-tools/lxi-tools#32-lxi
- tldr page: https://tldr.inbrowser.app/pages/linux/lxi
