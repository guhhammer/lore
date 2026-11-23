# dd

파일을 변환하고 복사.

## Usage
- `dd if=경로/대상/파일.iso of=/dev/usb_drive status=progress` : isohybrid 파일(예: `archlinux-xxx.iso`)에서 부팅 가능한 USB 드라이브를 만들고 진행 상황을 표시
- `dd bs=4194304 conv=fsync if=/dev/source_drive of=/dev/dest_drive` : 블록 크기가 4 MiB인 다른 드라이브에 복제하고 명령이 끝나기 전 쓰기 버퍼를 비움
- `dd bs=100 count=1 if=/dev/urandom of=경로/대상/임의의_파일` : 커널 랜덤 드라이버를 사용하여 임의 바이트를 가진 파일을 생성
- `dd bs=1024 count=1000000 if=/dev/zero of=경로/대상/파일_1GB` : 디스크의 순차적인 쓰기 성능을 벤치마킹
- `dd if=/dev/drive_device of=경로/대상/파일.img status=progress` : 시스템 백업을 생성하여, IMG 파일에 저장하고 (나중에 `if`와 `of`를 교체해 복원 가능), 진행 상황을 표시

## Links
- More information: https://manned.org/dd.1p
