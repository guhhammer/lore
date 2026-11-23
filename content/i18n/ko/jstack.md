# jstack

Java 스택 추적 도구.

## Usage
- `jstack 자바_pid` : java 프로세스의 모든 스레드에 대한 Java 스택 추적 출력
- `jstack -m 자바_pid` : java 프로세스의 모든 스레드에 대한 혼합 모드(Java/C++) 스택 추적 출력
- `jstack /usr/bin/java 파일.core` : java 코어 덤프에서 스택 추적 출력

## Links
- More information: https://manned.org/jstack
