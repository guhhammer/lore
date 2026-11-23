# fly

concourse-ci용 명령줄 도구.

## Usage
- `fly -t 타겟_이름 login -n 팀_이름 -c https://ci.example.com` : concourse 대상으로 인증 및 저장
- `fly targets` : 타겟 나열
- `fly -t 타겟_이름 pipelines` : 파이프라인 나열
- `fly -t 타겟_이름 set-pipeline -c pipeline.yml -p 파이프라인_이름` : 파이프라인 업로드 또는 업데이트
- `fly -t 타겟_이름 unpause-pipeline -p 파이프라인_이름` : 파이프라인 일시중지 해제
- `fly -t 타겟_이름 get-pipeline -p 파이프라인_이름` : 파이프라인 구성 표시
- `fly -t 타겟_이름 sync` : fly의 로컬 사본 업데이트
- `fly -t 타겟_이름 destroy-pipeline -p 파이프라인_이름` : 파이프라인 삭제

## Links
- More information: https://concourse-ci.org/fly.html
