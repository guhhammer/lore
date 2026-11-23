# bootctl

EFI 펌웨어 부트 설정을 제어하고 부트 로더를 관리합니다.

## Usage
- `bootctl status` : 시스템 펌웨어 및 부트 로더에 대한 정보 표시
- `bootctl list` : 사용 가능한 모든 부트 로더 항목 표시
- `sudo bootctl reboot-to-firmware true` : 다음 부팅 시 시스템 펌웨어로 부팅하도록 플래그 설정 (`sudo systemctl reboot --firmware-setup`과 유사)
- `bootctl --esp-path=/경로/대상/efi_시스템_파티션/` : EFI 시스템 파티션 경로 지정 (기본값은 `/efi/`, `/boot/` 또는 `/boot/efi`)
- `sudo bootctl install` : EFI 시스템 파티션에 `systemd-boot` 설치
- `sudo bootctl remove` : EFI 시스템 파티션에서 설치된 모든 버전의 `systemd-boot` 제거

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/bootctl.html
