# stty

터미널 장치 인터페이스의 옵션을 설정하거자 조회.

## Usage
- `stty size` : 현재 터미널 크기를 표시
- `stty -a` : 현재 터미널의 모든 설정을 표시
- `stty rows|cols 개수` : 행 또는 열의 개수를 설정
- `stty -F 경로/대상/장치_파일 speed` : 장치의 실제 전송 속도를 확인
- `stty sane` : 현재 터미널에 대해 모든 모드를 적절한 기본값으로 재설정
- `stty raw|cooked` : raw 모드와 일반 모드 간 전환
- `stty -echo|echo` : 문자 입력 echo 비활성화 또는 활성화
- `stty --help` : 도움말 표시

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/stty-invocation.html
