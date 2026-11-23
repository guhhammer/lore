# beanstalkd

단순하고 일반적인 work-queue 서버.

## Usage
- `beanstalkd` : beanstalkd를 시작하고, 11300 포트로 듣기
- `beanstalkd -l ip_address -p port_number` : 사용자가 지정한 포트 및 주소에서 beanstalkd 듣기 시작
- `beanstalkd -b path/to/persistence_directory` : work queue를 디스크에 저장하고 유지
- `beanstalkd -b path/to/persistence_directory -f 500` : 500밀리초마다 지속성있는 디렉토리에 동기화

## Links
- More information: https://manned.org/beanstalkd
