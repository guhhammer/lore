# maim

스크린샷 유틸리티.

## Usage
- `maim 경로/대상/스크린샷.png` : 스크린샷을 캡처하여 지정된 경로에 저장
- `maim -s 경로/대상/스크린샷.png` : 선택한 영역의 스크린샷 캡처
- `maim -s | xclip -se c -t image/png` : 선택한 영역의 스크린샷을 캡처하여 클립보드에 저장 (`xclip` 필요)
- `maim -i $(xdotool getactivewindow) 경로/대상/스크린샷.png` : 현재 활성 창의 스크린샷 캡처 (`xdotool` 필요)

## Links
- More information: https://manned.org/maim
