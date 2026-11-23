# bundletool

Manipulate Android Application Bundles. Some subcommands such as `validate` have their own usage documentation.

## Usage
- `bundletool help subcommand` : display help for a subcommand
- `bundletool build-apks --bundle path/to/bundle.aab --ks path/to/key.keystore --ks-key-alias key_alias --output path/to/file.apks` : generate APKs from an application bundle (prompts for keystore password)
- `bundletool build-apks --bundle path/to/bundle.aab --ks path/to/key.keystore --ks-key-alias key_alias --ks-pass pass:the_password --output path/to/file.apks` : generate APKs from an application bundle giving the keystore password
- `bundletool build-apks --bundle path/to/bundle.aab --mode universal --ks path/to/key.keystore --ks-key-alias key_alias --output path/to/file.apks` : generate APKs including only one single APK for universal usage
- `bundletool install-apks --apks path/to/file.apks` : install the right combination of APKs to an emulator or device
- `bundletool get-size total --apks path/to/file.apks` : estimate the download size of an application
- `bundletool get-device-spec --output path/to/file.json` : generate a device specification JSON file for an emulator or device
- `bundletool validate --bundle path/to/bundle.aab` : verify a bundle and display detailed information about it

## Categories
files, hardware, monitoring

## Links
- More information: https://developer.android.com/tools/bundletool
- tldr page: https://tldr.inbrowser.app/pages/common/bundletool
