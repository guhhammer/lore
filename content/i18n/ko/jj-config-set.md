# jj config set

설정 옵션에 지정한 값을 설정. 값은 TOML 표현식으로 지정.

## Usage
- `jj config s --user user.name "이름"` : 사용자 수준 설정 파일에 사용자 이름 설정
- `jj config s --user user.email "이메일"` : 사용자 수준 설정 파일에 사용자 이메일 설정
- `jj config s --repo 이름 값` : 저장소 수준 설정 파일에 설정 옵션 설정
- `jj config s --workspace 이름 값` : 작업 공간 수준 설정 파일에 설정 옵션 설정
- `jj config s --user 이름 true|false` : boolean 설정 옵션 설정

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-config-set
