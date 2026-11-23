# raw

Unix 원시 문자 장치를 블록 장치에 바인드합니다.

## Usage
- `raw /dev/raw/raw1 /dev/block_device` : 원시 문자 장치를 블록 장치에 바인드
- `raw /dev/raw/raw1` : 새로운 바인딩을 설정하는 대신 기존 바인딩 조회
- `raw -qa` : 바인드된 모든 원시 장치 조회

## Links
- More information: https://manned.org/raw.8
