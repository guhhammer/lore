# xattr

확장 파일 시스템 속성을 다루는 유틸리티.

## Usage
- `xattr -l 파일` : 주어진 파일의 key:value 확장 속성 나열
- `xattr -w 속성_키 속성_값 파일` : 주어진 파일에 속성 작성
- `xattr -d com.apple.quarantine 파일` : 주어진 파일에서 속성 삭제
- `xattr -c 파일` : 주어진 파일에서 모든 확장 속성 삭제
- `xattr -rd 속성_키 폴더` : 주어진 폴더에서 속성을 재귀적으로 삭제

## Links
- More information: https://keith.github.io/xcode-man-pages/xattr.1.html
