# aa-cleanprof

사용되지 않는 규칙을 제거하여 AppArmor 보안 프로파일을 정리하는 도구.

## Usage
- `sudo aa-cleanprof 프로파일_이름` : 프로파일에서 사용되지 않는 규칙 제거
- `sudo aa-cleanprof 프로파일1 프로파일2 ...` : 여러 프로파일을 한 번에 정리
- `sudo aa-cleanprof -d /경로/대상/프로파일 프로파일_이름` : 프로파일이 있는 디렉터리 지정
- `sudo aa-cleanprof -s 프로파일_이름` : 프롬프트 없이 조용히 실행
- `sudo aa-cleanprof --no-reload 프로파일_이름` : 정리 후 프로파일 재로드 방지
- `aa-cleanprof -h` : 도움말 표시

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-cleanprof.8
