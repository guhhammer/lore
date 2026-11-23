# clang-check

기본적인 오류를 검사하고, Clang의 추상 구문 트리(AST)와 함께 작업. Clang의 LibTooling의 일부, C/C++ 코드를 디버깅 및 분석하는 데 유용.

## Usage
- `clang-check 경로/대상/파일.cpp --` : 소스 파일에 대해 기본 검사 실행
- `clang-check 경로/대상/파일.cpp -ast-dump --` : 디버깅을 위해 Abstract Syntax Tree 출력
- `clang-check 경로/대상/파일.cpp -ast-dump -ast-dump-filter FunctionName` : 이름으로 AST 필터링
- `clang-check 경로/대상/파일.cpp -ast-print --` : AST를 사람이 읽기 좋게 출력

## Links
- More information: https://manned.org/clang-check
