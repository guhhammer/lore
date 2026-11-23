# bluetoothctl

Spravuje Bluetooth zařízení.

## Usage
- `bluetoothctl` : vstoupit do `bluetoothctl` shellu
- `bluetoothctl devices` : vypsat všechna známá zařízení
- `bluetoothctl power on|off` : zapnout nebo vypnout Bluetooth ovladač
- `bluetoothctl -t 10 scan on` : vyhledat dostupné zařízeni po dobu 10 sekund
- `bluetoothctl pair mac_addresa` : spárovat se zařízením
- `bluetoothctl connect|disconnect mac_addresa` : připojit se k nebo odpojit se od spárovaného zařízení
- `bluetoothctl trust mac_addresa` : povolit zařízení připojit se zpět
- `bluetoothctl remove mac_addresa` : smazat zařízení

## Links
- More information: https://manned.org/bluetoothctl
