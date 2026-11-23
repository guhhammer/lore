# hf

Hugging Face Hub와 상호작용. 로그인, 로컬 캐시 관리, 파일 다운로드 또는 업로드.

## Usage
- `hf auth login` : hugging Face Hub에 로그인
- `hf auth whoami` : 로그인한 사용자의 이름을 표시
- `hf auth logout` : 로그아웃
- `hf env` : 환경에 관한 정보를 출력
- `hf download --repo-type 저장소_타입 저장소_id 파일명1 파일명2 ...` : 저장소에서 파일을 다운로드하고 경로를 출력 (전체 저장소를 다운로드하려면 파일 이름을 생략)
- `hf upload --repo-type 저장소_타입 저장소_id 경로/대상/로컬_파일_또는_디렉터리 경로/대상/저장소_파일_또는_디렉터리` : hugging Face에 전체 폴더 또는 파일 업로드
- `hf cache ls` : 캐시를 스캔하여 다운로드한 저장소와 디스크 사용량을 확인

## Links
- More information: https://huggingface.co/docs/huggingface_hub/guides/cli
