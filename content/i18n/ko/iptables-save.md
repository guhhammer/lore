# iptables-save

`iptables` IPv4 설정 저장. IPv6의 경우 `ip6tables-save` 사용.

## Usage
- `sudo iptables-save` : `iptables` 설정 출력
- `sudo iptables-save -t 테이블` : 특정 t테이블의 `iptables` 설정 출력
- `sudo iptables-save -f 경로/대상/파일` : `iptables` 설정을 f파일에 저장

## Links
- More information: https://manned.org/iptables-save
