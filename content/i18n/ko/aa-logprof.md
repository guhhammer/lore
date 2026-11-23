# aa-logprof

기록된 위반 로그를 기반으로 AppArmor 보안 프로파일을 대화형으로 업데이트하는 도구.

## Usage
- `sudo aa-logprof` : 시스템 로그를 기반으로 대화형으로 프로파일 검토 및 업데이트
- `sudo aa-logprof -d /경로/대상/프로파일` : appArmor 프로파일 디렉터리 지정
- `sudo aa-logprof -f /경로/대상/로그파일` : 기본 로그 대신 특정 로그 파일 사용
- `sudo aa-logprof -m "로그_마커_텍스트"` : 지정한 마커 이전 로그 항목 무시
- `aa-logprof -h` : 도움말 표시

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-logprof.8
