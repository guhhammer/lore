# yj

Convert between YAML, TOML, JSON, and HCL, preserving map order.

## Usage
- `yj <file.yml -y` : convert yAML to JSON (default) from `stdin` and write the result to `stdout`
- `yj <file.toml -ty` : convert tOML to yAML
- `yj <file.json -jti` : convert jSON to tOML with indentation
- `yj <file.hcl -cj` : convert HcL to jSON
- `yj <file.yml -ycn` : convert yAML to HcL ignoring inf/naN conversion
- `yj -v` : display version

## Categories
text, files

## Links
- More information: https://github.com/sclevine/yj
- tldr page: https://tldr.inbrowser.app/pages/common/yj
