# qdbus

Inter-Process Communication (IPC) and Remote Procedure Calling (RPC) mechanism originally developed for Linux.

## Usage
- `qdbus` : list available service names
- `qdbus service_name` : list object paths for a specific service
- `qdbus service_name /path/to/object` : list methods, signals, and properties available on a specific object
- `qdbus service_name /path/to/object method_name argument1 argument2 ...` : execute a specific method passing arguments and display the returned value
- `qdbus org.kde.Solid.PowerManagement /org/kde/Solid/PowerManagement/Actions/BrightnessControl org.kde.Solid.PowerManagement.Actions.BrightnessControl.brightness` : display the current brightness value in a KDE Plasma session
- `qdbus org.kde.Solid.PowerManagement /org/kde/Solid/PowerManagement/Actions/BrightnessControl org.kde.Solid.PowerManagement.Actions.BrightnessControl.setBrightness 5000` : set a specific brightness to a KDE Plasma session
- `qdbus org.kde.kglobalaccel /component/kmix invokeShortcut "increase_volume"` : invoke volume up shortcut in a KDE Plasma session
- `qdbus org.kde.Shutdown /Shutdown logout|logoutAndReboot|logoutAndShutdown` : gracefully log out and then do nothing, reboot, or shut down

## Categories
desktop, system, processes

## Links
- More information: https://doc.qt.io/qt-6/qtdbus-index.html
- tldr page: https://tldr.inbrowser.app/pages/common/qdbus
