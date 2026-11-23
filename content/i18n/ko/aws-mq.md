# aws mq

AWS에서 메시지 브로커를 정의하고 운영.

## Usage
- `aws mq create-broker --host-instance-type 인스턴스_타입 --broker-name 브로커_이름 --engine-type ACTIVEMQ|RABBITMQ --publicly-accessible|--no-publicly-accessible` : 브로커 생성
- `aws mq list-brokers` : 모든 브로커 목록 표시
- `aws mq describe-broker --broker-id 브로커_아이디` : 특정 브로커 상세 정보 표시

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/mq/
