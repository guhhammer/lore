# jarsigner

Java 아카이브(`.jar`) 파일 서명 및 검증 도구.

## Usage
- `jarsigner 경로/대상/파일.jar 키스토어_별칭` : `.jar` 파일 서명
- `jarsigner -sigalg 알고리즘 경로/대상/파일.jar 키스토어_별칭` : 특정 알고리즘으로 `.jar` 파일 서명
- `jarsigner -verify 경로/대상/파일.jar` : `.jar` 파일의 서명 검증

## Links
- More information: https://docs.oracle.com/en/java/javase/25/docs/specs/man/jarsigner.html
