# gcloud

Google Cloud Platform을 위한 공식 CLI 도구.

## Usage
- `gcloud config list` : 활성 구성에서 모든 속성 나열
- `gcloud auth login` : google 계정에 로그인
- `gcloud config set project 프로젝트_이름` : 활성 프로젝트 설정
- `gcloud compute ssh 사용자@인스턴스` : 가상 머신 인스턴스에 SSH 접속
- `gcloud compute instances list` : 프로젝트 내 모든 Google Compute Engine 인스턴스 표시 (기본적으로 모든 영역의 인스턴스가 나열됨)
- `gcloud container clusters get-credentials 클러스터_이름` : 적절한 자격 증명으로 kubeconfig 파일을 업데이트하여 `kubectl`을 특정 Google Kubernetes Engine (GKE) 클러스터에 연결
- `gcloud components update` : 모든 `gcloud` 구성 요소 업데이트
- `gcloud help 명령` : 특정 명령에 대한 도움말 표시

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud
