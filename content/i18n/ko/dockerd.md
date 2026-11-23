# dockerd

Docker 컨테이너를 시작하고 관리하는 지속적인 프로세스.

## Usage
- `dockerd` : docker 데몬 실행
- `dockerd -H unix://경로/대상/tmp.sock -H tcp://ip_address` : docker 데몬을 실행하고 특정 소켓(UNIX 및 TCP)을 수신하도록 설정
- `dockerd -p 경로/대상/pid_파일` : 특정 데몬 PID 파일로 실행
- `dockerd -D` : 디버그 모드로 실행
- `dockerd -l debug|info|warn|error|fatal` : 특정 로그 레벨로 실행

## Links
- More information: https://docs.docker.com/reference/cli/dockerd/
