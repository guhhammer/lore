# wlc

Weblate 인스턴스의 현지화 프로젝트를 관리.

## Usage
- `wlc -c 경로/대상/파일 list-projects` : 설정 파일을 사용하여 프로젝트 목록 표시
- `wlc -u url -k 키 ls 프로젝트` : API URL과 API 키를 직접 지정하여 프로젝트의 컴포넌트 목록 표시
- `wlc -f text|csv|json|html ls 프로젝트/컴포넌트` : 지정한 형식으로 컴포넌트의 번역 목록 표시
- `wlc stats 프로젝트` : 프로젝트 통계 출력
- `wlc -h` : 도움말 표시

## Links
- More information: https://docs.weblate.org/en/latest/wlc.html#commands
