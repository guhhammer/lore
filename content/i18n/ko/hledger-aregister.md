# hledger aregister

한 계좌의 거래 내역과 잔액을 한 줄로 표시.

## Usage
- `hledger areg assets:bank:checking` : `assets:bank:checking` 계좌의 거래 내역과 잔액 표시
- `hledger areg savings` : `*savings*`라는 이름의 첫 번째 계좌의 거래 내역과 잔액 표시
- `hledger areg checking -C -w 120` : 지정된 너비로 체크 계좌의 정리된 거래 내역 표시
- `hledger areg checking --forecast` : 예측 규칙의 거래를 포함하여 체크 계좌의 내역 표시

## Links
- More information: https://hledger.org/hledger.html#aregister
