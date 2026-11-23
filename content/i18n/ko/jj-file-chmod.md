# jj file chmod

`jj` 저장소의 경로에 실행 권한 비트를 설정하거나 제거.

## Usage
- `jj file chmod x 경로/대상/파일` : 작업 복사본의 파일에 실행 권한 설정
- `jj file chmod n 경로/대상/파일` : 작업 복사본의 파일에서 실행 권한 제거 (일반 파일로 설정)
- `jj file chmod x 경로/대상/파일1 경로/대상/파일2 ...` : 여러 파일에 실행 권한 설정
- `jj file chmod -r 리비전 x 경로/대상/파일` : 지정한 리비전의 파일에 실행 권한 설정

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-file-chmod
