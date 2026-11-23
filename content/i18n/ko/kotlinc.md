# kotlinc

Kotlin 컴파일러.

## Usage
- `kotlinc` : REPL (대화형 셸) 시작
- `kotlinc 경로/대상/파일.kt` : kotlin 파일 컴파일
- `kotlinc 경로/대상/파일1.kt 경로/대상/파일2.kt ...` : 여러 Kotlin 파일 컴파일
- `kotlinc -script 경로/대상/파일.kts` : 특정 Kotlin Script 파일 실행
- `kotlinc 경로/대상/파일.kt -include-runtime -d 경로/대상/파일.jar` : kotlin 파일을 Kotlin 런타임 라이브러리가 포함된 독립 실행형 jar 파일로 컴파일

## Links
- More information: https://kotlinlang.org/docs/compiler-reference.html#common-options
