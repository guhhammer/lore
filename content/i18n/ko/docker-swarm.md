# docker swarm

컨테이너 오케스트레이션 도구.

## Usage
- `docker swarm init` : 스웜 클러스터 초기화
- `docker swarm join-token worker|manager` : 매니저 또는 워커에 합류할 수 있는 토큰 표시
- `docker swarm join --token 토큰 매니저_노드_url:2377` : 새로운 노드를 클러스터에 합류
- `docker swarm leave` : 스웜에서 워커 제거 (워커 노드 내부에서 실행)
- `docker swarm ca` : 현재 CA 인증서를 PEM 형식으로 표시
- `docker swarm ca --rotate` : 현재 CA 인증서를 갱신하고 새 인증서 표시
- `docker swarm update --cert-expiry 시간h분m초s` : 노드 인증서의 유효 기간 변경

## Links
- More information: https://docs.docker.com/engine/swarm/
