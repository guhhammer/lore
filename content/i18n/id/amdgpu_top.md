# amdgpu_top

Tool untuk menampilkan penggunaan GPU AMD dan metrik perangkat keras menggunakan driver AMDGPU.

## Usage
- `amdgpu_top --list` : tampilkan daftar perangkat AMDGPU
- `amdgpu_top -p` : lakukan dump semua proses GPU dan penggunaan memori per-proses
- `amdgpu_top --pci "0000:01:00.0"` : pilih GPU spesifik berdasarkan bus PCI
- `amdgpu_top` : jalankan monitor TUI interaktif
- `amdgpu_top --gui` : jalankan monitor GUI
- `amdgpu_top --smi` : jalankan tampilan TUI sederhana mirip SMI

## Links
- More information: https://github.com/Umio-Yasuno/amdgpu_top#usage
