# grub-install

GRUB을 장치에 설치.

## Usage
- `grub-install --target=i386-pc 경로/대상/장치` : BIOS 시스템에 GRUB 설치
- `grub-install --target=x86_64-efi --efi-directory=경로/대상/efi_폴더 --bootloader-id=GRUB` : UEFI 시스템에 GRUB 설치
- `grub-install --target=x86_64-efi --efi-directory=경로/대상/efi_폴더 --modules="part_gpt part_msdos"` : 특정 모듈을 사전 로드하여 GRUB 설치
