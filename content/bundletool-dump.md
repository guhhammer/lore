# bundletool dump

Manipulate Android Application Bundles.

## Usage
- `bundletool dump manifest --bundle path/to/bundle.aab` : display the `AndroidManifest.xml` of the base module
- `bundletool dump manifest --bundle path/to/bundle.aab --xpath /manifest/@android:versionCode` : display a specific value from the `AndroidManifest.xml` using XPath
- `bundletool dump manifest --bundle path/to/bundle.aab --module name` : display the `AndroidManifest.xml` of a specific module
- `bundletool dump resources --bundle path/to/bundle.aab` : display all the resources in the application bundle
- `bundletool dump resources --bundle path/to/bundle.aab --resource type/name` : display the configuration for a specific resource
- `bundletool dump resources --bundle path/to/bundle.aab --resource 0x7f0e013a --values` : display the configuration and values for a specific resource using the ID
- `bundletool dump config --bundle path/to/bundle.aab` : display the contents of the bundle configuration file

## Categories
files, desktop, development

## Links
- More information: https://developer.android.com/tools/bundletool
- tldr page: https://tldr.inbrowser.app/pages/common/bundletool-dump
