# bootc

컨테이너 이미지를 기반으로 부팅 및 업그레이드를 수행하는 도구. OCI/Docker 컨테이너 이미지를 사용하여 트랜잭션 방식의 안전하게(롤백 가능) 현재 시스템 위에서 OS 업데이트를 관리하는 방식.

## Usage
- `bootc status` : 부트로더에 표시될 순서대로 배포 상태 확인
- `bootc upgrade --check` : 사용 가능한 업데이트 확인
- `bootc upgrade --apply` : 업데이트 준비 후 재부팅하여 적용
- `bootc switch 이미지` : OS 베이스를 새로운 컨테이너 이미지로 변경
- `bootc rollback` : 이전 ostree 배포로 롤백
- `bootc edit` : 시스템 설정에 트랜잭션 변경 적용

## Links
- More information: https://manned.org/bootc
