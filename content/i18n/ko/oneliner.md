# oneliner

영어를 OpenAI, Claude, 로컬 LLM을 사용해 쉘 명령으로 변환하는 도구(사용자 지식 대체가 아닌, 학습을 돕는 것을 목표로 함).

## Usage
- `oneliner "find all jpg files larger than 10MB"` : 영어로부터 쉘 명령 생성
- `oneliner -e "delete node_modules recursively"` : 명령이 수행하는 작업 설명
- `oneliner -c "compress all pdfs"` : 생성된 명령을 클립보드에 복사
- `oneliner -b "list all active network connections"` : 명령을 교육적인 관점에서, 자세히 분석하여 설명
- `oneliner -r "list files larger than 1GB"` : 생성된 명령 실행 (사용에 주의가 필요)
- `oneliner -i "delete temporary files in /tmp"` : 생성된 명령 실행 전에 대화형으로 확인 요청

## Links
- More information: https://github.com/dorochadev/oneliner#-usage-flags
