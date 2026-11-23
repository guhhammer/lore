# aws batch

AWS Batch를 통해 배치 컴퓨팅 워크로드를 실행합니다.

## Usage
- `aws batch list-jobs --job-queue queue_name` : 실행 중인 배치 작업 목록
- `aws batch create-compute-environment --compute-environment-name compute_environment_name --type type` : 컴퓨팅 환경 생성
- `aws batch create-job-queue --job-queue-name queue_name --priority priority --compute-environment-order compute_environment` : 배치 작업 큐 생성
- `aws batch submit-job --job-name job_name --job-queue job_queue --job-definition job_definition` : 작업 제출
- `aws batch describe-jobs --jobs jobs` : 배치 작업 목록 설명
- `aws batch cancel-job --job-id job_id --reason reason` : 작업 취소

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/batch/
