# flex

어휘 분석기(Lexical analyzer)를 생성하는 도구. 어휘 분석기 명세를 입력받아, 이를 구현한 C 코드를 생성.

## Usage
- `flex analyzer.l` : lex 파일로부터 어휘 분석기를 생성하여, `lex.yy.c` 파일에 저장
- `flex -t analyzer.l` : 생성된 어휘 분석기를 `stdout`로 출력
- `flex analyzer.l -o analyzer.c` : 출력 파일 지정
- `flex -B analyzer.l` : 대화형 스캐너 대신 배치 스캐너 생성
- `cc path/to/lex.yy.c -o 실행파일` : lex가 생성한 C 파일 컴파일

## Links
- More information: https://manned.org/flex
