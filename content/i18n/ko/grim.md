# grim

Wayland 컴포지터에서 이미지를 캡처(스크린샷)합니다.

## Usage
- `grim` : 모든 출력의 스크린샷 캡처
- `grim -o DP-1 경로/대상/출력_파일` : 특정 출력의 스크린샷 캡처
- `grim -g "x_위치,y_위치 너비x높이"` : 특정 영역의 스크린샷 캡처
- `grim -g "$(slurp)"` : 특정 영역을 선택하고 스크린샷 캡처 (slurp 사용)
- `grim "경로/대상/파일.png"` : 사용자 정의 파일명 사용
- `grim - | 클립보드_관리자` : 스크린샷을 캡처하고 클립보드에 복사

## Links
- More information: https://manned.org/grim
