# clang

C, C++, Objective-C 소스 파일을 컴파일. 수정 없이 GCC 대체(drop-in replacement)로 사용할 수 있음. LLVM의 일부.

## Usage
- `clang 경로/대상/소스1.c 경로/대상/소스2.c ... -o 경로/대상/결과_실행파일` : 여러 소스 파일을 하나의 실행 파일로 컴파일
- `clang 경로/대상/소스.c -Wall -o 결과_실행파일` : 모든 오류 및 경고 출력 활성화
- `clang 경로/대상/소스.c -Wall -g -Og -o 경로/대상/결과_실행파일` : 일반적인 경고 표시, 디버그 심볼 포함, 디버깅에 영향을 주지 않는 최적화 적용
- `clang 경로/대상/소스.c -o 경로/대상/결과_실행파일 -I경로/대상/header -L경로/대상/라이브러리 -l라이브러리_이름` : 다른 경로의 라이브러리 및 헤더 포함
- `clang -S -emit-llvm 경로/대상/소스.c -o 경로/대상/출력파일.ll` : 소스 코드를 LLVM 중간 표현(IR)으로 컴파일
- `clang -c 경로/대상/소스.c` : 소스 코드를 링킹 없이 오브젝트 파일로 컴파일
- `clang 경로/대상/소스.c -O1|2|3|fast -o 경로/대상/결과_실행파일` : 성능 최적화를 적용해 컴파일
- `clang --version` : 버전 표시

## Links
- More information: https://clang.llvm.org/docs/ClangCommandLineReference.html
