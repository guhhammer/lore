# jhsdb

Java 프로세스에 연결하거나 충돌한 Java 가상 머신의 코어 덤프를 분석하기 위해 사후 디버거를 실행.

## Usage
- `jhsdb jstack --pid pid` : java 프로세스의 스택 및 잠금 정보 출력
- `jhsdb clhsdb --core 경로/대상/core_dump --exe 경로/대상/jdk/bin/java` : 코어 덤프를 인터랙티브 디버그 모드에서 열기
- `jhsdb debugd --pid pid --serverid 선택적_고유_id` : 원격 디버그 서버 시작
- `jhsdb clhsdb --pid pid` : 프로세스에 인터랙티브 디버그 모드로 연결

## Links
- More information: https://manned.org/jhsdb
