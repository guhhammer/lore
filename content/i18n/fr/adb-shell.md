# adb shell

Android Debug Bridge Shell: Exécute une commande shell sur une instance d'émulateur Android ou un appareil Android.

## Usage
- `adb shell` : démarre une session shell interactive sur l'émulateur/l'appareil
- `adb shell getprop` : récupère toutes les propriétés depuis un émulateur ou un appareil
- `adb shell pm reset-permissions` : remet toutes les permissions courante à leurs valeurs par défaut
- `adb shell pm revoke paquet permission` : révoque une permission dangereuse pour une application
- `adb shell input keyevent code` : déclenche un événement clé
- `adb shell pm clear paquet` : nettoie les données d'une application sur un émulateur ou un appareil
- `adb shell am start -n paquet/activité` : démarre une activité sur un émulateur ou un appareil
- `adb shell am start -W -c android.intent.category.HOME -a android.intent.action.MAIN` : démarre une activité maison depuis un émulateur ou un appareil

## Links
- More information: https://developer.android.com/tools/adb
