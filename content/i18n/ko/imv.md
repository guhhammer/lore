# imv

wayland와 X11을 지원하는 CLI 이미지 뷰어, tiling window manager에 최적화되어 있음. Photoshop (PSD)을 포함한 다양한 이미지 형식을 지원.

## Usage
- `imv 경로/대상/이미지1.ext 경로/대상/이미지2.ext ...` : 여러 이미지 보기
- `imv -f 경로/대상/이미지.ext` : 전체 화면 모드로 이미지 보기
- `imv -r --slideshow 경로/대상/디렉터리` : 지정한 디렉터리의 이미지를 재귀적으로(recursively) 탐색하여 슬라이드 쇼 실행
- `find . -type f -name "*.svg" | imv` : `stdin`으로 전달된 여러 이미지 보기
- `imv -t 10 경로/대상/디렉터리` : 디렉터리의 이미지를 10초 간격으로 슬라이드 쇼 실행
- `curl -Osw '%{filename_effective}\n' 'http://www.example.com/[1-10].jpg' | imv` : 웹의 여러 이미지 보기

## Links
- More information: https://sr.ht/~exec64/imv/
