# hledger

강력하고 사용하기 쉬운 텍스트 기반 회계 앱.

## Usage
- `hledger add` : 새로운 거래를 대화형으로 기록하고 기본 저널 파일에 저장
- `hledger import 경로/대상/bank.csv` : `bank.csv`에서 새로운 거래를 가져오고, `bank.csv.rules`를 사용하여 변환
- `hledger print -f 경로/대상/prices-2024.journal -f 경로/대상/prices-2023.journal` : 여러 지정된 저널 파일에서 모든 거래를 출력
- `hledger accounts -t --types` : 계정과 그 유형을 계층 구조로 모두 표시
- `hledger bs -E -t --no-elide` : 자산 및 부채 계정 잔액을 0을 포함하여 계층적으로 표시
- `hledger is -M -T -A --sort -2` : 월별 수입/지출/총액을 가장 큰 것부터 2단계로 요약하여 표시
- `hledger areg assets:bank:checking` : `assets:bank:checking` 계정의 거래 및 진행 중인 잔액을 표시
- `hledger print assets:cash | hledger -f - -I aregister expenses:food` : `assets:cash` 계정에서 음식에 소비한 금액을 표시

## Links
- More information: https://hledger.org/hledger.html
