# lprm

서버의 대기 중인 인쇄 작업 취소.

## Usage
- `lprm` : 기본 프린터에서 현재 작업 취소
- `lprm -h 서버:포트 작업_id` : 특정 서버의 작업 취소
- `lprm -E 작업_id1 작업_id2 ...` : 서버에 암호화된 연결로 여러 작업 취소
- `lprm -` : 모든 작업 취소
- `lprm -P 대상/인스턴스` : 특정 프린터 또는 클래스의 현재 작업 취소

## Links
- More information: https://openprinting.github.io/cups/doc/man-lprm.html
