# dtc

The Device Tree Compiler, a tool for recompiling device trees between formats.

## Usage
- `dtc -I dts -O dtb -o path/to/output_file.dtb path/to/input_file.dts` : compile a device tree source `.dts` file into a binary device tree blob `.dtb` file
- `dtc -@ -I dts -O dtb -o path/to/output_file.dtbo path/to/input_file.dts` : compile a device tree source `.dts` file into a binary device tree blob overlay `.dtbo` file
- `dtc -I dtb -O dts -o path/to/output_file.dts path/to/input_file.dtb` : decompile a device tree blob `.dtb` file into a readable device tree source `.dts` file
- `dtc -I fs -O dts /proc/device-tree` : decompile the current device tree from the system into a readable device tree source `.dts` file

## Categories
hardware, files, development

## Links
- More information: https://manned.org/dtc
- tldr page: https://tldr.inbrowser.app/pages/common/dtc
