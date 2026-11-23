# adb reverse

Android Debug Bridge Reverse: Transfère des connections réseaux depuis une instance d'émulateur Android ou un appareil Android.

## Usage
- `adb reverse --list` : affiche la liste de toutes les connections réseaux qui sont transféré depuis les émulateurs ou les appareils
- `adb reverse tcp:port_distant tcp:port_local` : transfère un port TCP depuis un émulateur ou un appareil vers localhost
- `adb reverse --remove tcp:port_distant` : supprime une connection socket en cours depuis un émulateur ou un appareil
- `adb reverse --remove-all` : supprime toutes les connections socket depuis tous les émulateurs et appareils

## Links
- More information: https://developer.android.com/tools/adb
