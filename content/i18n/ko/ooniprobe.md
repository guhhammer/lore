# ooniprobe

네트워크 간섭의 열린 관측소 (OONI). 웹사이트 및 앱 차단 여부를 테스트하세요. 네트워크의 속도 및 성능을 측정하세요.

## Usage
- `ooniprobe list` : 수행된 모든 테스트 나열
- `ooniprobe list 7` : 특정 테스트에 대한 정보 표시
- `ooniprobe run all` : 사용 가능한 모든 테스트 실행
- `ooniprobe run performance` : 특정 테스트 수행
- `ooniprobe run websites --input https://ooni.org/` : 특정 웹사이트의 가용성 확인
- `ooniprobe run websites --input-file 경로/대상/my-websites.txt` : 파일에 나열된 모든 웹사이트의 가용성 확인
- `ooniprobe show 9` : JSON 형식으로 테스트에 대한 자세한 정보 표시

## Links
- More information: https://ooni.org/support/ooni-probe-cli/
