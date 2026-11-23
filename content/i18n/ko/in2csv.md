# in2csv

다양한 표 데이터 형식을 CSV로 변환. csvkit에 포함.

## Usage
- `in2csv data.xls` : XLS 파일을 CSV로 변환
- `in2csv data.dbf >data.csv` : DBF 파일을 CSV 파일로 변환
- `in2csv --sheet=시트_이름 data.xlsx` : 특정 시트를 XLSX 파일에서 CSV로 변환
- `cat data.json | in2csv -f json >data.csv` : JSON 파일을 in2csv로 파이프

## Links
- More information: https://csvkit.readthedocs.io/en/latest/scripts/in2csv.html
