# ukify

Build and sign Unified Kernel Images (UKIs).

## Usage
- `ukify build --linux path/to/vmlinuz --initrd path/to/initrd.img --cmdline "root=UUID=xxxx"` : build a Unified Kernel Image
- `ukify build --linux path/to/vmlinuz --initrd path/to/initrd1.img --initrd path/to/initrd2.img --cmdline "root=UUID=xxxx"` : build a UKI with multiple initrd files
- `ukify build --linux path/to/vmlinuz --initrd path/to/initrd.img --cmdline "root=UUID=xxxx" --output path/to/output.efi` : build a UKI, specifying the output filename
- `ukify build --linux path/to/vmlinuz --initrd path/to/initrd.img --microcode path/to/microcode.img --cmdline "root=UUID=xxxx"` : build a UKI including CPU microcode
- `ukify build --linux path/to/vmlinuz --initrd path/to/initrd.img --cmdline "root=UUID=xxxx" --secureboot-private-key path/to/private_key.pem --secureboot-certificate path/to/certificate.pem` : sign the UKI for Secure Boot
- `ukify inspect path/to/uki.efi` : inspect a UKI file
- `ukify genkey --secureboot-certificate-validity 365` : generate signing keys
- `ukify -h` : display help

## Categories
files, development, system

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/ukify.html
- tldr page: https://tldr.inbrowser.app/pages/linux/ukify
