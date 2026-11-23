# kubectl autoscale

kubernetes 클러스터 부하에 따라 Pod 수를 자동으로 조정하는 오토스케일러 생성.

## Usage
- `kubectl autoscale deploy deployment_이름 --min 최소_replicas --max 최대_replicas` : 목표 CPU 사용률 없이 Deployment 자동 스케일링 설정
- `kubectl autoscale deploy deployment_이름 --max 최대_replicas --cpu-percent 대상_cpu` : 목표 CPU 사용률을 기준으로 Deployment 자동 스케일링 설정

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_autoscale/
