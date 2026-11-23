# gcloud config

`gcloud`의 다양한 구성 관리.

## Usage
- `gcloud config set 속성 값` : 현재 구성에 대한 속성(예: compute/zone) 정의
- `gcloud config get 속성` : `gcloud` 속성의 값 가져오기
- `gcloud config list` : 현재 구성의 모든 속성 표시
- `gcloud config configurations create 구성_이름` : 주어진 이름으로 새 구성 만들기
- `gcloud config configurations list` : 사용 가능한 모든 구성 목록 표시
- `gcloud config configurations activate 구성_이름` : 주어진 이름의 기존 구성으로 전환

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud/reference/config
