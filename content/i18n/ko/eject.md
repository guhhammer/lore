# eject

CD, 플로피 디스크 및 테이프 드라이브를 꺼냅니다.

## Usage
- `eject -d` : 기본 장치 표시
- `eject` : 기본 장치 꺼내기
- `eject /dev/cdrom` : 특정 장치 꺼내기 (기본 순서는 cd-rom, scsi, 플로피 및 테이프입니다)
- `eject -T /dev/cdrom` : 장치 트레이가 열려 있는지 닫혀 있는지 토글
- `eject -r /dev/cdrom` : CD 드라이브 꺼내기
- `eject -f /mnt/floppy` : 플로피 드라이브 꺼내기
- `eject -q /mnt/tape` : 테이프 드라이브 꺼내기

## Links
- More information: https://manned.org/eject
