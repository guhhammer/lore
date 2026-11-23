# kaggle kernels

Kaggle 커널 관리.

## Usage
- `kaggle k list` : 모든 커널 목록 표시
- `kaggle k files 커널_이름` : 커널 출력 파일 목록 표시
- `kaggle k init -p 경로/대상/디렉터리` : 커널용 메타데이터 파일 초기화 (기본값: 현재 디렉터리)
- `kaggle k push -p 경로/대상/디렉터리` : 새 코드를 커널에 업로드하고 실행
- `kaggle k pull 커널_이름 -p 경로/대상/디렉터리` : 커널 가져오기
- `kaggle k output 커널_이름` : 최신 커널 실행 결과 데이터 가져오기
- `kaggle k status 커널_이름` : 최신 커널 실행 상태 표시

## Links
- More information: https://github.com/Kaggle/kaggle-api/blob/main/docs/README.md#kernels
