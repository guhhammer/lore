# kwriteconfig5

Write KConfig entries for KDE Plasma.

## Usage
- `kwriteconfig5 --group group_name --key key value` : set a global configuration key
- `kwriteconfig5 --file path/to/file --group group_name --key key value` : set a key in a specific configuration file
- `kwriteconfig5 --group group_name --key key --delete` : delete a key
- `kwriteconfig5 --file startkderc --group General --key systemdBoot true` : use systemd to start the Plasma session when available
- `kwriteconfig5 --file ~/.config/kwinrc --group Windows --key BorderlessMaximizedWindows true` : hide the title bar when a window is maximized (like Ubuntu)
- `kwriteconfig5 --file ~/.config/kwinrc --group ModifierOnlyShortcuts --key Meta "org.kde.kglobalaccel,/component/krunner_desktop,org.kde.kglobalaccel.Component,invokeShortcut,_launch"` : configure KRunner to open with the Meta (Command/Windows) global hotkey
- `kwriteconfig5 --help` : display help

## Categories
desktop, files, users

## Links
- More information: https://userbase.kde.org/KDE_System_Administration/Configuration_Files
- tldr page: https://tldr.inbrowser.app/pages/linux/kwriteconfig5
