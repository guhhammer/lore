# az cognitiveservices

Azure Cognitive Services 계정을 관리하는 명령어. `azure-cli`의 일부 (`az`).

## Usage
- `az cognitiveservices account create -n 계정_이름 -g 리소스_그룹 --kind api_이름 -l location --sku sku_이름 --yes` : 특정 지역에 확인 없이 API Cognitive Services 계정 생성
- `az cognitiveservices account list-usage -n 계정_이름 -g 리소스_그룹` : azure Cognitive Services 계정 사용량 조회
- `az cognitiveservices account deployment create -n 계정_이름 -g 리소스_그룹 --deployment-name 배포_이름 --model-name 모델_이름 --model-version "모델_버전" --model-format 포맷_이름` : azure Cognitive Services 계정에 배포 생성
- `az cognitiveservices account commitment-plan create -n 계정_이름 -g 리소스_그룹 --commitment-plan-name "plan_name" --hosting-model "호스팅_모델" --plan-type "플랜_타입" --auto-renew false|true` : azure Cognitive Services 계정에 커밋 플랜 생성
- `az cognitiveservices account commitment-plan delete -g 리소스_그룹 -n 계정_이름 --commitment-plan-name "플랜_이름"` : azure Cognitive Services 계정에서 커밋 플랜 삭제

## Links
- More information: https://learn.microsoft.com/cli/azure/cognitiveservices
