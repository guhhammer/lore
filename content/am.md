# am

Android activity manager.

## Usage
- `am start -n com.android.settings/.Settings` : start the activity with a specific component and package name
- `am start -a android.intent.action.VIEW -d tel:123` : start an intent action and pass data to it
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : start an activity matching a specific action and category
- `am to-uri -a android.intent.action.VIEW -d tel:123` : convert an intent to a URI
- `am start -W -c android.intent.category.HOME -a android.intent.action.MAIN` : start the home activity on an emulator or device

## Notes
An Android command: run it on a device or emulator, usually through `adb shell`.

## Categories
android

## Links
- More information: https://developer.android.com/tools/adb#am
- tldr page: https://tldr.inbrowser.app/pages/android/am
