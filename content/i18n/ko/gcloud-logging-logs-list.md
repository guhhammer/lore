# gcloud logging logs list

Google Cloud 프로젝트에서 로그 목록을 나열합니다. 모니터링 및 분석을 위한 사용 가능한 로그 식별에 유용합니다.

## Usage
- `gcloud logging logs list` : 현재 프로젝트의 모든 로그 나열
- `gcloud logging logs list --bucket=버킷_아이디 --location=위치` : 특정 로그 버킷 및 위치의 모든 로그 나열
- `gcloud logging logs list --bucket=버킷_아이디 --location=위치 --view=뷰_아이디` : 로그 버킷의 특정 뷰에 대한 모든 로그 나열
- `gcloud logging logs list --filter="표현식"` : 필터 표현식을 사용하여 로그 나열
- `gcloud logging logs list --limit=숫자` : 지정된 수의 로그 나열
- `gcloud logging logs list --sort-by="필드_이름"` : 특정 필드를 기준으로 오름차순 또는 내림차순(`~`는 내림차순)으로 정렬하여 로그 나열
- `gcloud logging logs list --sort-by="필드1,~필드2"` : 여러 필드를 기준으로 정렬하여 로그 나열
- `gcloud logging logs list --verbosity=debug` : 추가 세부 정보를 보여주는 자세한 출력으로 로그 나열

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud/reference/logging/logs/list
