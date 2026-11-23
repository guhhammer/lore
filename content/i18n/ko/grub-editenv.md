# grub-editenv

GRUB 환경 변수를 편집.

## Usage
- `grub-editenv /boot/grub/grubenv set default=Ubuntu` : 기본 부팅 항목 설정 (부팅 항목이 이미 존재한다고 가정)
- `grub-editenv /boot/grub/grubenv list timeout` : `timeout` 변수의 현재 값 표시
- `grub-editenv /boot/grub/grubenv unset saved_entry` : `saved_entry` 변수를 기본값으로 재설정
- `grub-editenv /boot/grub/grubenv list kernel_cmdline` : 커널 명령줄에 "quiet splash" 추가

## Links
- More information: https://www.gnu.org/software/grub/manual/grub/grub.html
