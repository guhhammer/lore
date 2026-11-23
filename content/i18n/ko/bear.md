# bear

`clang` 도구용 컴파일 데이터베이스를 생성하는 도구.

## Usage
- `bear -- make` : 빌드 명령을 실행하여 `compile_commands.json`을 생성
- `bear --output 경로/대상/컴파일_명령어.json -- make` : 사용자 정의 출력 파일 이름으로 컴파일 데이터베이스 생성
- `bear --append -- make` : 기존 `compile_commands.json` 파일에 결과를 추가
- `bear --verbose -- make` : 자세한 출력을 얻으려면, 상세 모드로 실행
- `bear --force-preload -- make` : `bear`가 명령 차단을 위해 사전 로드 방법을 사용하도록 강제

## Links
- More information: https://github.com/rizsotto/Bear
