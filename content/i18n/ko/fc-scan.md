# fc-scan

글꼴 파일 또는 디렉터리를 검사해 발견된 각 글꼴의 패턴 정보를 출력.

## Usage
- `fc-scan 경로/대상/글꼴_파일` : 글꼴 파일 검사
- `fc-scan 경로/대상/디렉터리` : 글꼴 디렉터리를 재귀적으로 검사
- `fc-scan 경로/대상/폰트1.ttf 경로/대상/폰트2.otf ...` : 여러 글꼴 파일 또는 디렉터리 검사
- `fc-scan -b 경로/대상/글꼴_파일` : 글꼴 크기 정보를 제외하고 결과 출력
- `fc-scan -f "%{family}\n" 경로/대상/글꼴_파일` : 사용자 지정 출력 형식으로 결과 출력
- `fc-scan -h` : 도움말 표시
- `fc-scan -V` : 버전 표시

## Links
- More information: https://manned.org/fc-scan
