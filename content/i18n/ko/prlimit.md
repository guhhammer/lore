# prlimit

프로세스 리소스의 소프트 및 하드 제한을 가져오거나 설정합니다. 프로세스 ID와 하나 이상의 리소스를 지정하면 prlimit이 제한을 검색 및/또는 수정하려고 시도합니다.

## Usage
- `prlimit` : 실행 중인 부모 프로세스의 모든 현재 리소스 제한 값을 표시
- `prlimit -p pid_번호` : 지정된 프로세스의 모든 현재 리소스 제한 값을 표시
- `prlimit -n10 명령` : 사용자 지정 열린 파일 수 제한으로 명령 실행

## Links
- More information: https://manned.org/prlimit
