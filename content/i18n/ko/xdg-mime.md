# xdg-mime

XDG 표준에 따라 MIME 유형을 조회하고 관리.

## Usage
- `xdg-mime query filetype 경로/대상/파일` : 파일의 MIME 유형 표시
- `xdg-mime query default image/png` : PNG 파일을 여는 기본 애플리케이션 표시
- `xdg-mime query default $(xdg-mime query filetype 경로/대상/파일)` : 특정 파일을 여는 기본 애플리케이션 표시
- `xdg-mime default imv.desktop image/png image/jpeg` : PNG 및 JPEG 이미지를 여는 기본 애플리케이션을 imv로 설정

## Links
- More information: https://portland.freedesktop.org/doc/xdg-mime.html
