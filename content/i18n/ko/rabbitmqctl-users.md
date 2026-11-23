# rabbitmqctl-users

RabbitMQ 사용자와 해당 사용자의 권한 및 태그를 관리.

## Usage
- `rabbitmqctl list_users` : 모든 사용자 목록 표시
- `rabbitmqctl add_user 사용자명 비밀번호` : 비밀번호를 지정하여 새로운 사용자 추가
- `rabbitmqctl delete_user 사용자명` : 기존 사용자 삭제
- `rabbitmqctl change_password 사용자명 새로운_비밀번호` : 사용자 비밀번호 변경
- `rabbitmqctl set_permissions -p vhost 사용자명 read write configure` : 지정한 가상 호스트에서 사용자의 권한 설정
- `rabbitmqctl clear_permissions -p vhost 사용자명` : 지정한 가상 호스트에서 사용자의 모든 권한 제거
- `rabbitmqctl set_user_tags 사용자명 태그1 태그2 ...` : 사용자에게 하나 이상의 태그(예: `administrator`) 할당

## Links
- More information: https://www.rabbitmq.com/docs/management
