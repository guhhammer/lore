# firejail

Linux의 내장 기능을 사용하여 프로세스를 안전하게 컨테이너로 샌드박스화합니다.

## Usage
- `sudo firecfg` : 데스크톱 환경에 firejail 통합
- `firejail firefox` : 제한된 Mozilla Firefox 열기
- `firejail --net=eth0 --ip=192.168.1.244 /etc/init.d/apache2 start` : 알려진 인터페이스와 주소에서 제한된 Apache 서버 시작
- `firejail --list` : 실행 중인 샌드박스 나열
- `firejail --netstats` : 실행 중인 샌드박스의 네트워크 활동 나열
- `firejail --shutdown=7777` : 실행 중인 샌드박스 종료
- `firejail --seccomp --private --private-dev --private-tmp --protocol=inet firefox --new-instance --no-remote --safe-mode --private-window` : 인터넷 탐색을 위한 제한된 Firefox 세션 실행
- `firejail --hosts-file=~/myhosts curl http://mysite.arpa` : 사용자 정의 호스트 파일 사용(`/etc/hosts` 파일 무시)

## Links
- More information: https://manned.org/firejail
