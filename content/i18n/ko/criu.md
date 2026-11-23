# criu

실행 중인 애플리케이션 또는 프로세스 트리를 일시 중단하여 저장한 뒤, 나중에 복원.

## Usage
- `sudo criu check` : 현재 커널이 CRIU에 필요한 기능을 지원하는지 확인
- `sudo criu dump -t 프로세스_아이디 -D 경로/대상/디렉터리` : 지정한 ID의 프로세스를 체크포인트하고, 상태를 지정한 디렉터리에 저장
- `sudo criu restore -D 경로/대상/디렉터리` : 이전에 저장한 이미지 파일에서 프로세스 복원
- `sudo criu dump -t 프로세스_아이디 -D 경로/대상/디렉터리 -R` : 프로세스를 체크포인트한 후 종료하지 않고 계속 실행

## Links
- More information: https://manned.org/criu
