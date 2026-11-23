# snmpget

SNMP 프로토콜을 사용해 값을 조회.

## Usage
- `snmpget -v 버전 -c 커뮤니티 ip_주소 oid` : SNMP 에이전트에서 단일 값 조회
- `snmpget -v 버전 -c 커뮤니티 -O f ip_address oid` : 전체 Object Identifier (OID) 경로 표시
- `snmpget -h` : 도움말 표시

## Links
- More information: https://manned.org/snmpget
