# swapon

디바이스와 파일을 스왑에 사용할 수 있도록 활성화합니다.

## Usage
- `swapon` : 스왑 정보를 표시
- `sudo swapon 경로/대상/파일` : 주어진 스왑 영역 활성화
- `sudo swapon -a` : `/etc/fstab`에 지정된 모든 스왑 영역(단, `noauto` 옵션이 있는 것은 제외) 활성화
- `sudo swapon -L 레이블` : 레이블로 스왑 파티션 활성화

## Links
- More information: https://manned.org/swapon.8
