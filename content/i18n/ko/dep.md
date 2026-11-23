# dep

PHP 애플리케이션 배포. 참고: 동일한 이름의 Go 명령어 `dep`은 더 이상 사용 되지 않으며 유지 보수되지 않음(archived).

## Usage
- `dep init` : 로컬 경로에서 대화형으로 deployer를 초기화 (`--template=template`으로 프레임워크 템플릿 사용 가능)
- `dep deploy 호스트명` : 애플리케이션을 원격 호스트에 배포
- `dep rollback` : 이전 정상 동작 릴리스로 롤백
- `dep ssh 호스트명` : SSH를 통해 원격 호스트에 접속
- `dep list` : 사용 가능한 명령 목록을 표시
- `dep run "명령어"` : 원격 호스트에서 임의의 명령을 실행
- `dep help 명령어` : 명령어에 대한 도움말 표시

## Links
- More information: https://deployer.org/docs/8.x/cli
