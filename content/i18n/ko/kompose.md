# kompose

Docker Compose 애플리케이션을 Kubernetes로 변환.

## Usage
- `kompose up -f docker-compose.yml` : 도커화된 애플리케이션을 Kubernetes에 배포
- `kompose down -f docker-compose.yml` : kubernetes에서 서비스/배포 인스턴스 삭제
- `kompose convert -f docker-compose.yml` : docker-compose 파일을 Kubernetes 리소스 파일로 변환

## Links
- More information: https://github.com/kubernetes/kompose
