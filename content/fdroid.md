# fdroid

F-Droid build tool. F-Droid is an installable catalog of FOSS (Free and Open Source Software) applications for the Android platform.

## Usage
- `fdroid build app_id` : build a specific app
- `fdroid build app_id --server` : build a specific app in a build server VM
- `fdroid publish app_id` : publish the app to the local repository
- `fdroid install app_id` : install the app on every connected device
- `fdroid lint --format app_id` : check if the metadata is formatted correctly
- `fdroid rewritemeta app_id` : fix the formatting automatically (if possible)

## Categories
development, packages

## Links
- More information: https://f-droid.org/en/docs/Building_Applications/
- tldr page: https://tldr.inbrowser.app/pages/common/fdroid
