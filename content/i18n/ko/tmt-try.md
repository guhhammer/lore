# tmt try

테스트와 환경을 빠르게 실험할 수 있는 명령어.

## Usage
- `tmt try` : 기본 provision 방식으로 빠르게 실험 (현재 디렉터리에 테스트 없음)
- `cd 경로/대상/테스트_디렉터리 && tmt try` : 현재 디렉터리에서 테스트 실행
- `tmt try rhel-9` : 특정 운영체제를 사용
- `tmt try fedora@container` : 사용자 지정 이미지와 provision 방식 선택
- `tmt try -t feature` : 사용자 정의 필터로 테스트 선택
- `tmt try -a` : 게스트를 준비하고 사용자 입력 대기
- `tmt try -l` : 묻지 않고 바로 게스트에 로그인
- `tmt try --help` : 도움말 표시

## Links
- More information: https://tmt.readthedocs.io/en/stable/stories/cli.html#try
