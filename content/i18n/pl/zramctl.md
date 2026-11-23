# zramctl

Tworzenie i kontrola urządzeń zram. Użyj `mkfs` lub `mkswap` aby sformatować urządzenia zram na partycje.

## Usage
- `lsmod | grep -i zram` : sprawdzenie, czy zram jest włączony
- `sudo modprobe zram` : włączenie zram z dynamiczną liczbą urządzeń (użyj `zramctl` aby skonfigurować urządzenia dalej)
- `sudo modprobe zram num_devices=2` : włączenie zram z dokładnie 2 urządzeniami
- `sudo zramctl -f -s 2GB -a lz4` : znalezienie i inicjalizacja następnego wolnego urządzenia zram jako 2 GB napęd wirtualny z użyciem kompresji LZ4
- `sudo zramctl` : wyświetlenie aktualnie zainicjalizowanych urządzeń

## Links
- More information: https://manned.org/zramctl
