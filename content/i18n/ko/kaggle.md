# kaggle

Python 3으로 구현된 공식 Kaggle CLI.

## Usage
- `kaggle config view` : 현재 설정 값 표시
- `kaggle c download competition -f 경로/대상/파일` : 대회 데이터셋에서 특정 파일 다운로드
- `kaggle c list -s 검색어` : 검색어와 일치하는 대회 목록 표시
- `kaggle c files 대회` : 특정 대회에서 사용 가능한 파일 목록 표시
- `kaggle c submit 대회 -f 경로/대상/submission.csv -m "메시지"` : 메시지와 함께 대회에 파일 제출
- `kaggle d list -s 검색어` : 검색어와 일치하는 데이터셋 목록 표시
- `kaggle d download 소유자/데이터셋_이름` : 데이터셋의 모든 파일 다운로드
- `kaggle k list -s 검색어` : 검색어와 일치하는 커널(노트북) 목록 표시

## Links
- More information: https://github.com/Kaggle/kaggle-api/blob/main/docs/README.md
