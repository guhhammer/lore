# rpicam-still

Raspberry Pi 카메라를 사용하여 사진을 촬영하고 저장하며, `rpicam-jpeg`에서 누락된 레거시 기능을 포함합니다.

## Usage
- `rpicam-still -e bmp|png|rgb|yuv420 -o 경로/대상/파일.ext` : 다른 인코딩 방식으로 사진 촬영
- `rpicam-still -r -o 경로/대상/파일.jpg` : RAW 이미지 촬영
- `rpicam-still -o 경로/대상/파일.jpg --shutter 100000` : 100초 노출 이미지 촬영
