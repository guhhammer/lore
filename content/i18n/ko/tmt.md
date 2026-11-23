# tmt

테스트를 생성, 실행, 디버깅하기 위한 테스트 관리 도구. `run`, `try`와 같은 하위 명령어는 사용법 문서를 자체적으로 가짐.

## Usage
- `tmt` : 사용 가능한 테스트, 플랜, 스토리 목록 출력
- `tmt init` : tmt 파일/프로젝트 구조 초기화
- `tmt test create -t beakerlib --link verifies:issue#1234` : 템플릿과 링크를 사용해 새로운 테스트 생성
- `tmt test|plan|story ls 패턴` : 테스트, 플랜, 스토리 목록 출력
- `tmt -c arch=aarch64 test show` : 특정 컨텍스트에서 테스트 메타데이터 상세 출력
- `tmt lint` : tmt 파일을 스펙에 맞게 검증
- `tmt tests ls -f tag:foo -f tier:0` : filter 사용
- `tmt --help` : 도움말 표시

## Links
- More information: https://tmt.readthedocs.io/en/stable/examples.html
