# hledger incomestatement

보고 기간 동안 수익 유입과 비용 유출 표시. 금액은 일반적인 재무제표처럼 정상적인 양수 기호로 표시됩니다.

## Usage
- `hledger is` : 수익과 비용(수익 및 비용 계정의 변화) 표시
- `hledger is -M` : 매월 수익과 비용 표시
- `hledger is -MTAS -2` : 매월 수익/비용/총액을 가장 큰 것부터 두 단계로 요약하여 표시
- `hledger is -MTAS -2 -o is.html` : 위 명령의 단축형으로, `is.html`에 HTML 출력 생성

## Links
- More information: https://hledger.org/hledger.html#incomestatement
