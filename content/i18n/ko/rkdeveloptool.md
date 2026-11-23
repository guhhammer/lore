# rkdeveloptool

Rockchip 기반 컴퓨터 장치의 부팅 펌웨어를 플래시, 덤프 및 관리. USB를 통해 연결하기 전에 장치를 Maskrom/Bootrom 모드로 전환해야 합니다. 일부 하위 명령은 루트 권한으로 실행해야 할 수 있습니다.

## Usage
- `rkdeveloptool ld` : 연결된 모든 Rockchip 기반 플래시 장치 list
- `rkdeveloptool db 경로/대상/부트로더.bin` : 지정된 파일에서 bootloader를 다운로드 및 설치하도록 장치를 download 모드로 초기화
- `rkdeveloptool ul 경로/대상/부트로더.bin` : 부트loader 소프트웨어를 새 버전으로 update
- `rkdeveloptool wl 초기_섹터 경로/대상/이미지.img` : GPT 형식의 플래시 파티션에 이미지를 작성하고 초기 저장소 섹터를 지정 (일반적으로 `0x0` 또는 `0`)
- `rkdeveloptool wlx 파티션_이름 경로/대상/이미지.img` : 사용자 친화적인 이름으로 플래시 파티션에 쓰기
- `rkdeveloptool rd` : reset/재부팅 device, Maskrom/Bootrom 모드에서 나와 선택된 플래시 파티션으로 부팅

## Links
- More information: https://github.com/rockchip-linux/rkdeveloptool
