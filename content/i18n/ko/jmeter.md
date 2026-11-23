# jmeter

기능적 동작의 부하 테스트 및 성능 측정을 위해 설계된 오픈 소스 Java 애플리케이션.

## Usage
- `jmeter -n -t 경로/대상/파일.jmx` : GUI 없이 특정 테스트 플랜 실행
- `jmeter -n -t 경로/대상/파일.jmx -l 경로/대상/로그파일.jtl` : 특정 로그 파일을 사용하여 GUI 없이 테스트 플랜 실행
- `jmeter -n -t 경로/대상/파일.jmx -H- 127.0.0.1 -P 8888` : 특정 프록시를 사용하여 GUI 없이 테스트 플랜 실행
- `jmeter -n -t 경로/대상/파일.jmx -J 키='값'` : 특정 JMeter 속성을 사용하여 GUI 없이 테스트 플랜 실행

## Links
- More information: https://jmeter.apache.org/usermanual/get-started.html#options
