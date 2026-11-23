# ern

Electrode Native platform client.

## Usage
- `ern create-miniapp application_name` : create a new `ern` application (`MiniApp`)
- `ern run-ios|android` : run one or more `MiniApps` in the iOS/Android Runner application
- `ern create-container --miniapps /path/to/miniapp_directory --platform ios|android` : create an Electrode Native container
- `ern publish-container --publisher maven --platform android --extra '{"groupId":"com.walmart.ern","artifactId":"quickstart"}'` : publish an Electrode Native container to a local Maven repository
- `ern transform-container --platform ios --transformer xcframework` : transform an iOS container into a pre-compiled binary framework
- `ern platform versions` : list all installed versions of Electrode Native
- `ern platform config set logLevel trace|debug` : set a logging level

## Categories
containers

## Links
- More information: https://native.electrode.io/reference/index-6
- tldr page: https://tldr.inbrowser.app/pages/common/ern
