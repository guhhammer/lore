# rabbitmqctl-cluster

RabbitMQ 클러스터의 노드를 관리.

## Usage
- `rabbitmqctl cluster_status` : 클러스터 상태 표시
- `rabbitmqctl status` : 현재 노드의 상태 표시
- `rabbitmqctl -n 노드이름 start_app` : 지정한 노드에서 RabbitMQ 애플리케이션 시작
- `rabbitmqctl -n 노드이름 stop_app` : 지정한 노드에서 RabbitMQ 애플리케이션 중지
- `rabbitmqctl -n 노드이름 stop` : 지정한 RabbitMQ 노드 중지
- `rabbitmqctl -n 노드이름 reset` : 지정한 RabbitMQ 노드를 초기 상태로 재설정
- `rabbitmqctl join_cluster 노드이름` : 현재 노드를 기존 클러스터에 참여시킴

## Links
- More information: https://www.rabbitmq.com/docs/man/rabbitmqctl.8
