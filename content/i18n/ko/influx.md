# influx

InfluxDB 커멘드 라인 클라이언트.

## Usage
- `influx` : 자격증명 없이 localhost에서 실행되는 InfluxDB에 연결
- `influx -username 사용자명 -password ""` : 특정 사용자 이름으로 연결 (비밀번호를 묻는 메시지가 표시됨)
- `influx -host 호스트명` : 특정 호스트에 연결
- `influx -database 데이터베이스_이름` : 특정 데이터베이스 사용
- `influx -execute "influxql_명령"` : 주어진 명령을 실행
- `influx -execute "influxql_명령" -format json|csv|column` : 특정 형식으로 출력을 반환

## Links
- More information: https://docs.influxdata.com/influxdb/v1/tools/influx-cli/use-influx-cli/
