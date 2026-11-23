# fastboot

부트로더 모드에 있을 때 연결된 Android 장치와 통신 (ADB가 작동하지 않는 곳).

## Usage
- `fastboot oem unlock` : 부트로더 잠금해제
- `fastboot oem lock` : 부트로더 리로드
- `fastboot reboot bootloader` : 장치를 fastboot 모드에서 fastboot 모드로 재부팅
- `fastboot flash 경로/대상/파일.img` : 주어진 이미지를 조사
- `fastboot flash recovery 경로/대상/파일.img` : 사용자 정의 복구 이미지 조사
- `fastboot devices` : 연결된 장치 목록 나열
- `fastboot getvar all` : 장치의 모든 정보를 표시
