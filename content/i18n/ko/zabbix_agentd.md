# zabbix_agentd

서버 매개변수를 모니터링하는 데몬.

## Usage
- `zabbix_agentd` : 기본 설정 파일을 사용하여 에이전트 시작
- `zabbix_agentd -c 경로/대상/zabbix_에이전트_설정파일.conf` : 사용자 지정 설정 파일을 사용하여 에이전트 시작
- `zabbix_agentd -c 경로/대상/zabbix_에이전트_설정파일.conf -f` : 에이전트를 포그라운드에서 실행 (현재 터미널 세션에 연결된 상태로 유지)
- `zabbix_agentd -c 경로/대상/zabbix_에이전트_설정파일.conf -T` : 설정 파일 테스트
- `zabbix_agentd -c 경로/대상/zabbix_에이전트_설정파일.conf -t 아이템_키 -v` : 자세한 출력과 함께 지정한 항목 테스트
- `zabbix_agentd -c 경로/대상/zabbix_에이전트_설정파일.conf -R userparameter_reload` : 설정 파일에서 사용자 매개변수를 다시 불러오기 (런타임 제어)
- `zabbix_agentd -c 경로/대상/zabbix_에이전트_설정파일.conf -R log_level_increase|decrease` : 모든 프로세스의 로그 수준 높이기 또는 낮추기 (런타임 제어)
- `zabbix_agentd -h` : 도움말 표시

## Links
- More information: https://manned.org/zabbix_agentd
