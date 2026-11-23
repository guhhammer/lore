# colab

Google Colab을 위한 명령줄 인터페이스. `run`, `exec`와 같은 일부 하위 명령에는 자체 사용 설명서가 있습니다.

## Usage
- `colab new -s 세션_이름 --gpu T4|L4|G4|A100|H100` : 특정 GPU를 사용하여 새 세션을 할당
- `colab sessions` : 모든 활성 세션 나열
- `colab run --gpu T4|L4|A100|H100 경로/대상/스크립트.py` : 새로운 GPU VM에서 로컬 스크립트를 실행하고 완료 시 자동으로 VM 해제
- `colab exec -s 세션_이름 -f 경로/대상/파일.py|경로/대상/노트북.ipynb` : 특정 세션에서 로컬 Python 스크립트 또는 Jupyter 노트북 실행
- `colab console -s 세션_이름` : 원격 VM의 대화형 TTY 셸에 연결
- `colab drivemount -s 세션_이름` : 특정 세션에 Google Drive 마운트
- `colab download -s 세션_이름 경로/대상/원격_파일 경로/대상/로컬_대상` : 원격 세션에서 파일 다운로드
- `colab stop -s 세션_이름` : 특정 세션 중지 및 종료

## Links
- More information: https://github.com/googlecolab/google-colab-cli#command-index
