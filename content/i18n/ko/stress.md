# stress

CPU, 메모리 및 IO를 스트레스 테스트하는 Linux 시스템 도구.

## Usage
- `stress -c 4` : CPU 스트레스 테스트를 위해 4개의 워커 생성
- `stress -i 2 -t 5` : IO 스트레스 테스트를 위해 2개의 워커 생성하고 5초 후 타임아웃
- `stress -m 2 --vm-bytes 256M` : 메모리 스트레스 테스트를 위해 2개의 워커 생성 (각 워커는 256M 바이트 할당)
- `stress -d 2 --hdd-bytes 1GB` : write()/unlink()를 반복하는 2개의 워커 생성 (각 워커는 1G 바이트 씀)

## Links
- More information: https://manned.org/stress
