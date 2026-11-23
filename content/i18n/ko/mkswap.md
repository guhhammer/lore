# mkswap

디바이스나 파일에 Linux 스왑 영역을 설정합니다.

## Usage
- `sudo mkswap 경로/대상/파일` : 지정된 스왑 영역 설정
- `sudo mkswap -c 경로/대상/파일` : 스왑 영역을 생성하기 전에 파티션의 불량 블록 확인
- `sudo mkswap -L 레이블 /dev/sda1` : 파티션에 레이블 지정 (레이블을 사용하여 `swapon` 사용 가능)

## Links
- More information: https://manned.org/mkswap
