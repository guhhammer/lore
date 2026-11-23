# chflags

파일 또는 디렉토리 플래그 변경.

## Usage
- `chflags hidden 경로/대상/파일` : 파일에 `hidden` 플래그 설정
- `chflags nohidden 경로/대상/파일` : 파일에서 `hidden` 플래그 해제
- `chflags -R uchg 경로/대상/폴더` : 디렉토리에 대해 `uchg` 플래그를 재귀적으로 설정
- `chflags -R nouchg 경로/대상/폴더` : 디렉토리에 대해 `uchg` 플래그를 재귀적으로 해제

## Links
- More information: https://keith.github.io/xcode-man-pages/chflags.1.html
