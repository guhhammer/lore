# doctl

DigitalOcean 리소스를 관리.

## Usage
- `doctl auth init` : digitalOcean 인증
- `doctl account get` : 계정 정보 표시
- `doctl compute droplet create 이름 --size 크기 --image 이미지 --region 리전` : 새로운 Droplet
- `doctl compute droplet list` : 모든 Droplets 목록 표시
- `doctl compute ssh droplet_아이디` : droplet에 SSH 접속
- `doctl kubernetes cluster create 클러스터_이름` : kubernetes 클러스터 생성
- `doctl kubernetes cluster list` : kubernetes 클러스터 목록 표시
- `doctl databases list` : 관리형 데이터베이스 목록 표시

## Links
- More information: https://docs.digitalocean.com/reference/doctl/
