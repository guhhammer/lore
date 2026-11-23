# autorecon

다중 스레드 네트워크 정찰 도구로, 서비스의 자동 열거를 수행합니다.

## Usage
- `sudo autorecon 호스트_또는_ip1,호스트_또는_ip2,...` : 대상 호스트에 대해 정찰 수행 (`./results`에 자세한 스캔 결과가 저장됩니다)
- `sudo autorecon -t 경로/대상/파일` : 파일에 지정된 t대상에 대해 정찰 수행
- `sudo autorecon -o 경로/대상/결과 호스트_또는_ip1,호스트_또는_ip2,...` : 다른 디렉토리에 o출력 결과 저장
- `sudo autorecon -p T:21-25,80,443,U:53,B:123 호스트_또는_ip1,호스트_또는_ip2,...` : 특정 p포트 및 프로토콜로 스캔 제한 (`T`는 TCP, `U`는 UDP, `B`는 둘 다)

## Links
- More information: https://github.com/AutoRecon/AutoRecon
