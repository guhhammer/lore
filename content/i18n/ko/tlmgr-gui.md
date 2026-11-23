# tlmgr gui

`tlmgr`의 그래픽 사용자 인터페이스를 시작. `tlmgr gui`는 수동으로 설치해야 하는 `perl-tk` 패키지에 의존.

## Usage
- `sudo tlmgr gui` : `tlmgr`를 위한 GUI 시작
- `sudo tlmgr gui -background "#f39bc3"` : 배경색을 지정하여 GUI 시작
- `sudo tlmgr gui -foreground "#0ef3bd"` : 전경색을 지정하여 GUI 시작
- `sudo tlmgr gui -font "helvetica 18"` : 글꼴과 글꼴 크기를 지정하여 GUI 시작
- `sudo tlmgr gui -geometry 너비x높이-x위치+y위치` : 특정 크기를 설정하여 GUI 시작
- `sudo tlmgr gui -xrm xresource` : 임의의 X 리소스 문자열을 전달하여 GUI 시작

## Links
- More information: https://www.tug.org/texlive/doc/tlmgr.html#gui
