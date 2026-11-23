# tail

파일의 마지막 부분을 표시.

## Usage
- `tail -n 줄_수 경로/대상/파일` : 파일에서 마지막 'count' 줄 표시
- `tail -n +줄_수 경로/대상/파일` : 특정 줄 번호부터 파일 출력
- `tail -n 바이트_수 경로/대상/파일` : 주어진 파일의 끝에서 특정 바이트 수 출력
- `tail -f 경로/대상/파일` : 주어진 파일의 마지막 줄을 출력하고 `<Ctrl c>`를 누를 때까지 계속 읽기
- `tail -F 경로/대상/파일` : 파일이 접근할 수 없는 경우에도 `<Ctrl c>`를 누를 때까지 계속 읽기
- `tail -n 줄_수 -s 초 -f 경로/대상/파일` : '파일'의 마지막 'num' 줄을 표시하고 'n' 초마다 새로 고침

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tail-invocation.html
