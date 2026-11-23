# efibootmgr

UEFI 부트 매니저를 조작합니다.

## Usage
- `efibootmgr -u` : 부트 옵션과 해당 번호를 모두 나열
- `sudo efibootmgr -c -d /dev/sda -p 1 -l "\경로\대상\shell.efi" -L "UEFI Shell"` : UEFI Shell v2를 부트 옵션으로 추가
- `sudo efibootmgr -c -d /dev/sda -p 1 -l "\vmlinuz" -u "kernel_cmdline" -L "Linux"` : linux를 부트 옵션으로 추가
- `sudo efibootmgr -o 0002,0008,0001,0005` : 현재 부트 순서 변경
- `sudo efibootmgr -B -b 0008` : 부트 옵션 삭제

## Links
- More information: https://manned.org/efibootmgr
