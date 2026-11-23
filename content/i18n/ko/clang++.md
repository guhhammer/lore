# clang++

C++ 소스 파일을 컴파일. LLVM의 일부.

## Usage
- `clang++ 경로/대상/소스파일1.cpp 경로/대상/소스파일2.cpp ... -o 경로/대상/결과_실행파일` : 여러 소스 파일을 하나의 실행 파일로 컴파일
- `clang++ 경로/대상/소스파일.cpp -Wall -o 결과_실행파일` : 모든 오류 및 경고 출력 활성화
- `clang++ 경로/대상/소스파일.cpp -Wall -g -Og -o 경로/대상/결과_실행파일` : 일반적인 경고 표시, 디버그 심볼 포함, 디버깅에 영향을 주지 않는 최적화 적용
- `clang++ 경로/대상/소스파일.cpp -std=c++20 -o 경로/대상/결과_실행파일` : 컴파일에 사용할 언어 표준 지정
- `clang++ 경로/대상/소스파일.cpp -o 경로/대상/결과_실행파일 -I경로/대상/헤더_경로 -L경로/대상/라이브러리_경로 -l경로/대상/라이브러리_이름` : 다른 경로에 있는 헤더 및 라이브러리 포함
- `clang++ -S -emit-llvm 경로/대상/소스파일.cpp -o 경로/대상/결과파일.ll` : 소스 코드를 LLVM 중간 표현(IR)으로 컴파일
- `clang++ 경로/대상/소스파일.cpp -O1|2|3|fast -o 경로/대상/결과_실행파일` : 성능 최적화를 적용해 컴파일
- `clang++ --version` : 버전 표시

## Links
- More information: https://clang.llvm.org/docs/UsersManual.html#command-line-options
