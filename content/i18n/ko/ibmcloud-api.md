# ibmcloud api

IBM Cloud API 엔드포인트 설정 또는 조회.

## Usage
- `ibmcloud api` : 현재 API 엔드포인트 표시
- `ibmcloud api cloud.ibm.com` : API 엔드포인트를 `cloud.ibm.com`으로 설정
- `ibmcloud api private.cloud.ibm.com` : 프라이빗 API 엔드포인트 설정
- `ibmcloud api private.cloud.ibm.com --vpc` : 프라이빗 엔드포인트에 VPC 연결 사용
- `ibmcloud api https://cloud.ibm.com --skip-ssl-validation` : HTTP 요청의 SSL 검증 우회
- `ibmcloud api --unset` : API 엔드포인트 설정 제거

## Links
- More information: https://cloud.ibm.com/docs/cli?topic=cli-ibmcloud_cli#ibmcloud_api
