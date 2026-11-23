# mokutil

Secure Boot Machine Owner Keys (MOK)를 구성. Secure Boot 활성화/비활성화 또는 키 등록과 같은 일부 작업은 재부팅이 필요합니다.

## Usage
- `mokutil --sb-state` : secure Boot가 활성화되어 있는지 표시
- `mokutil --enable-validation` : secure Boot 활성화
- `mokutil --disable-validation` : secure Boot 비활성화
- `mokutil -l` : 등록된 키 나열
- `mokutil -i 경로/대상/키.der` : 새 키 등록
- `mokutil -N` : 등록될 키 나열
- `mokutil --set-verbosity true` : shim 자세히 설정

## Links
- More information: https://manned.org/mokutil
