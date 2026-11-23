# git fame

Git 저장소 기여도를 예쁘게 출력.

## Usage
- `git fame` : 현재 Git 저장소의 기여도 계산
- `git fame --excl "정규_표현식"` : 지정된 정규 표현식과 일치하는 파일/디렉토리 제외
- `git fame --since "3주_전|2021-05-13"` : 지정된 날짜 이후의 기여도 계산
- `git fame --format pipe|yaml|json|csv|tsv` : 지정된 형식으로 기여도 출력
- `git fame -t` : 파일 확장자별 기여도 표시
- `git fame -w` : 공백 변화 무시
- `git fame -C` : 파일 간의 줄 이동 및 복사 감지
- `git fame -M` : 파일 내의 줄 이동 및 복사 감지

## Links
- More information: https://manned.org/git-fame
