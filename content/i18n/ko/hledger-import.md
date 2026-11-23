# hledger import

하나 이상의 데이터 파일에서 새 거래를 가져와 주요 저널에 추가.

## Usage
- `hledger import 경로/대상/은행.csv` : `bank.csv.rules`를 사용하여 `bank.csv`에서 새 거래 가져오기
- `hledger import 경로/대상/은행1.csv 경로/대상/은행2.csv --dry-run` : 두 파일에서 가져올 내용을 보여주고 아무 작업도 하지 않기
- `hledger import --rules-file common.rules *.csv` : 모든 CSV 파일에서 새 거래 가져오기, 모든 파일에 동일한 규칙 사용
- `watchexec -- hledger -f 경로/대상/은행.csv print` : `bank.csv.rules`를 편집하면서 변환 오류 또는 결과 보기
- `hledger import --catchup 경로/대상/은행.csv` : `bank.csv`의 현재 데이터를 이미 가져온 것으로 표시
- `rm -f .latest.bank.csv` : `bank.csv`를 모두 새로 가져온 것으로 표시

## Links
- More information: https://hledger.org/hledger.html#import
