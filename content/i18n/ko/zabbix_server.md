# zabbix_server

Zabbix 소프트웨어의 핵심 데몬.

## Usage
- `zabbix_server` : 기본 설정 파일을 사용해 서버를 시작
- `zabbix_server -c 경로/대상/zabbix_서버.conf` : 사용자 지정 설정 파일을 사용하여 서버 시작
- `zabbix_server -c 경로/대상/zabbix_서버.conf -f` : 서버를 포그라운드에서 실행
- `zabbix_server -c 경로/대상/zabbix_서버.conf -T` : 설정 파일을 테스트하고 종료
- `zabbix_server -c 경로/대상/zabbix_서버.conf -R config_cache_reload` : 설정 캐시 다시 불러오기 (런타임 제어)
- `zabbix_server -c 경로/대상/zabbix_서버.conf -R housekeeper_execute` : housekeeper 실행 (런타임 제어)
- `zabbix_server -c 경로/대상/zabbix_서버.conf -R log_level_increase|decrease` : 모든 프로세스의 로그 수준 높이기 또는 낮추기 (런타임 제어)
- `zabbix_server -h` : 도움말 표시

## Links
- More information: https://manned.org/zabbix_server
