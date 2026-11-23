# nextflow

계산 파이프라인 실행. 주로 생물정보학 워크플로우에 사용됩니다.

## Usage
- `nextflow run main.nf -resume` : 파이프라인 실행, 이전 실행의 캐시된 결과 사용
- `nextflow run 사용자/저장소 -revision 릴리스_태그` : gitHub에서 원격 워크플로우의 특정 릴리스 실행
- `nextflow run 워크플로우 -work-dir 경로/대상/폴더 -with-report 보고서.html` : 중간 파일을 위한 작업 디렉토리를 지정하고 실행 보고서 저장
- `nextflow log` : 현재 디렉토리에서 이전 실행의 세부 정보 표시
- `nextflow clean -force 실행_이름` : 특정 실행의 캐시 및 중간 파일 제거
- `nextflow list` : 다운로드된 모든 프로젝트 나열
- `nextflow pull 사용자/저장소 -hub bitbucket` : bitbucket에서 원격 워크플로우의 최신 버전 가져오기
- `nextflow self-update` : nextflow 업데이트

## Links
- More information: https://www.nextflow.io/docs/latest/cli.html
