# phpcbf

phpcs에서 감지된 위반 사항 수정.

## Usage
- `phpcbf 경로/대상/폴더` : 지정된 디렉터리의 문제 수정 (기본적으로 PEAR 표준 사용)
- `phpcbf -i` : 설치된 코딩 표준 목록 표시
- `phpcbf 경로/대상/폴더 --standard 표준` : 검사할 코딩 표준 지정
- `phpcbf 경로/대상/폴더 --extensions 파일_확장자1,파일_확장자2,...` : 쉼표로 구분된 파일 확장자를 지정하여 스니핑할 때 포함
- `phpcbf 경로/대상/폴더 --bootstrap 경로/대상/파일1,경로/대상/파일2,...` : 처리 전에 로드할 쉼표로 구분된 파일 목록
- `phpcbf 경로/대상/폴더 -l` : 하위 디렉터리로 재귀하지 않음

## Links
- More information: https://github.com/squizlabs/PHP_CodeSniffer
