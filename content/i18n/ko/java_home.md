# java_home

`$JAVA_HOME`의 값을 반환하거나 이 변수를 사용하여 명령을 실행합니다.

## Usage
- `java_home --version 1.5+` : 특정 버전에 기반한 JVM 목록 나열
- `java_home --arch i386` : 특정 [arch]아키텍처에 기반한 JVM 목록 나열
- `java_home --datamodel Applets|WebStart|BundledApp|JNI|CommandLine` : 특정 작업에 기반한 JVM 목록 나열 (기본값은 `CommandLine`)
- `java_home --xml` : XML 형식으로 JVM 목록 나열
- `java_home --help` : 도움말 표시

## Links
- More information: https://www.unix.com/man-page/osx/1/java_home
