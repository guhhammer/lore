# crane export

컨테이너 이미지의 파일 시스템을 tarball로 내보냄.

## Usage
- `crane export 이미지_이름 -` : `stdout`에 tarball을 작성
- `crane export 이미지_이름 경로/대상/tarball` : 파일에 tarball 쓰기
- `crane export - 경로/대상/파일` : `stdin`에서 이미지 읽기
