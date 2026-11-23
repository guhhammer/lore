# nvram

펌웨어 변수를 조작합니다.

## Usage
- `nvram -p` : NVRAM에 저장된 모든 변수 p출력
- `nvram -xp` : NVRAM에 저장된 모든 변수를 xML 형식으로 p출력
- `sudo nvram 이름="값"` : 펌웨어 변수의 값 수정
- `sudo nvram -d 이름` : 펌웨어 변수 d삭제
- `sudo nvram -c` : 모든 펌웨어 변수 c지우기
- `sudo nvram -xf 경로/대상/파일.xml` : 특정 xML f파일에서 펌웨어 변수 설정

## Links
- More information: https://keith.github.io/xcode-man-pages/nvram.8.html
