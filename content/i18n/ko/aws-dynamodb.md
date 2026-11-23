# aws dynamodb

예측 가능한 성능과 원활한 확장성을 갖춘 빠른 NoSQL 데이터베이스인 AWS Dynamodb 데이터베이스를 조작.

## Usage
- `aws dynamodb create-table --table-name 테이블_이름 --attribute-definitions AttributeName=S,AttributeType=S --key-schema AttributeName=S,KeyType=HASH --provisioned-throughput ReadCapacityUnits=5,WriteCapacityUnits=5` : 테이블 생성
- `aws dynamodb list-tables` : dynamoDB의 모든 테이블 나열
- `aws dynamodb describe-table --table-name 테이블_이름` : 특정 테이블에 대한 세부정보 출력
- `aws dynamodb put-item --table-name 테이블_이름 --item '{"AttributeName": {"S": "value"\}\}'` : 테이블에 항목 ㅊ두가
- `aws dynamodb get-item --table-name 테이블_이름 --key '{"ID": {"N": "1"\}\}'` : 테이블에서 항목 검색
- `aws dynamodb update-item --table-name 테이블_이름 --key '{"ID": {"N": "1"\}\}' --update-expression "SET Name = :n" --expression-attribute-values '{":n": {"S": "Jane"\}\}'` : 테이블의 항목 업데이트
- `aws dynamodb scan --table-name 테이블_이름` : 테이블의 항목을 스캔
- `aws dynamodb delete-item --table-name 테이블_이름 --key '{"ID": {"N": "1"\}\}'` : 테이블에서 항목 제거

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/dynamodb/
