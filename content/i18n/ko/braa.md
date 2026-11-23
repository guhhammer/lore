# braa

여러 호스트를 동시에 스캔할 수 있는 초고속 대량 SNMP 스캐너.

## Usage
- `braa public@ip_address:.1.3.6.*` : 호스트의 SNMP 트리를 public 문자열로 탐색하여 `.1.3.6` 하위의 모든 OID 쿼리
- `braa public@ip_range:.1.3.6.1.2.1.1.6.0` : `ip_range`의 전체 서브넷에 대해 `system.sysLocation.0` 쿼리
- `braa private@ip_address:.1.3.6.1.2.1.1.6.0=s'workgroup'` : `system.sysLocation.0`의 값을 특정 워크그룹으로 설정 시도

## Links
- More information: https://manned.org/braa
