# adb logcat

Jeter une log des messages systèmes.

## Usage
- `adb logcat` : affiche les logs systèmes
- `adb logcat -e regex` : affiche les logs qui correspond à une `regex`
- `adb logcat tag:mode *:S` : affiche les logs pour un tag donné, dans un mode spécifique (Verbose, Debug, Info, Warning, Error, Fatal, Silent)
- `adb logcat ReactNative:V ReactNativeJS:V *:S` : affiche les logs pour des applications React Native en mode Verbose et mes sous Silence les autres tags
- `adb logcat *:W` : affiche les logs avec un niveau de priorité supérieur à Warning
- `adb logcat -v color` : colorie les logs (souvent utilisé avec des filtres)

## Links
- More information: https://developer.android.com/tools/logcat
