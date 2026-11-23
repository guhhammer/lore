# apparmor_parser

AppArmor 보안 프로파일을 로드, 컴파일, 관리하는 도구.

## Usage
- `sudo apparmor_parser -a 프로파일_파일` : 프로파일을 커널에 로드
- `sudo apparmor_parser -r 프로파일_파일` : 기존 프로파일을 교체
- `sudo apparmor_parser -R 프로파일_이름` : 커널에서 프로파일 제거
- `sudo apparmor_parser -C -r 경로/대상/프로파일` : complain 모드로 프로파일 로드(위반 로그만 기록, 차단하지 않음)
- `apparmor_parser -p -o 경로/대상/출력파일.cache -Q 경로/대상/프로파일` : 프로파일 전처리 (includes 해결) 후 바이너리 캐시 파일로 저장
- `apparmor_parser -p -S -Q 경로/대상/프로파일` : 프로파일 전처리 후 커널에 로드하지 않고 `stdout`으로 출력
- `sudo apparmor_parser -r -T 경로/대상/프로파일` : 캐시 읽기를 건너뛰고 프로파일 교체
- `sudo apparmor_parser -r -W -L /경로/대상/캐시 경로/대상/프로파일` : 프로파일 교체 및 캐시 재생성 후, 지정 디렉터리에 저장

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_apparmor_parser.8
