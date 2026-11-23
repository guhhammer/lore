# hledger balance

유연하고 일반적인 "합계" 보고서로, 숫자 데이터가 포함된 계정을 보여줍니다. 이는 기간별 잔액 변화, 종료 잔액, 예산 성과, 미실현 자본 이득 등을 포함할 수 있습니다.

## Usage
- `hledger bal` : 모든 시간 동안 모든 계좌의 모든 기입에서 잔액 변화를 표시
- `hledger bal expenses -t -2` : `*expenses*`로 명명된 계정에서, 상위 두 수준만 요약하여 트리 형태로 잔액 변화를 표시
- `hledger bal expenses -M -T -A -S --budget` : 매월 비용, 총계 및 평균을 표시하고, 총계로 정렬하며 월간 예산 목표 포함
- `hledger bal type:X -MTAS --budget -t -2 --no-elide` : 위와 유사하지만, `Expense` 유형으로 계정을 일치시키고, 지루한 계정을 생략하지 않고 두 개의 수준 트리로 표시
- `hledger bal -H -p 'quarterly in 2024' assets liabilities` : 종료 잔액을 2024년 분기별로, `*assets*` 또는 `*liabilities*`로 명명된 계정에서 표시 (시작일 이전의 기입 포함)
- `hledger bal -HQ date:2024 type:AL -ES -3` : 위와 유사하지만, 0 잔액도 표시하고, 총계로 정렬하여 세 개의 수준으로 요약
- `hledger bal -HVQ assets:investments` : 각 분기 말에 기본 통화로 투자 자산의 시장 가치를 표시
- `hledger bal --gain -Q assets:investments not:cryptocurrency` : 암호화폐가 아닌 투자 자산에 대한 각 분기의 시장 가격 변동으로 인한 미실현 자본 이득/손실 표시

## Links
- More information: https://hledger.org/hledger.html#balance
