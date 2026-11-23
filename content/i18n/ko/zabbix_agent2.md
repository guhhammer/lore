# zabbix_agent2

서버 매개변수를 모니터링하는 데몬.

## Usage
- `zabbix_agent2` : 기본 설정 파일을 사용해 에이전트 시작
- `zabbix_agent2 -c 경로/대상/zabbix_에이전트2.conf` : 사용자 지정 설정 파일을 사용하여 에이전트 시작
- `zabbix_agent2 -c 경로/대상/zabbix_에이전트2.conf -T` : 설정 파일을 테스트하고 종료
- `zabbix_agent2 -c 경로/대상/zabbix_에이전트2.conf -t 아이템_키 -v` : 자세한 출려과 함께 지정한 항목 테스트
- `zabbix_agent2 -c 경로/대상/zabbix_에이전트2.conf -R userparameter_reload` : 설정 파일에서 사용자 매개변수를 다시 불러오기 (런타임 제어)
- `zabbix_agent2 -c 경로/대상/zabbix_에이전트2.conf -R loglevel increase|decrease` : 로그 수준 높이기 또는 낮추기 (런타임 제어)
- `zabbix_agent2 -h` : 도움말 표시

## Links
- More information: https://manned.org/zabbix_agent2
