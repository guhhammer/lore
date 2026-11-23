# gosec

Go 소스 코드의 보안 취약점 검사.

## Usage
- `gosec ./...` : 현재 디렉터리의 Go 패키지를 재귀적으로 검사
- `gosec 경로/대상/패키지` : 지정한 Go 패키지 검사
- `gosec -fmt json|yaml|csv|html|sonarqube|golint|sarif|junit-xml -out 경로/대상/리포트 ./...` : go 패키지를 재귀적으로 검사하고 결과를 파일로 저장
- `gosec -include G101,G203,G401,... ./...` : 지정한 규칙만 사용하여 검사 (기본값: 모든 규칙)
- `gosec -exclude-dir 경로/대상/제외될_디렉터리 ./...` : 지정한 디렉터리를 검사 대상에서 제외
- `gosec -severity low|medium|high -confidence low|medium|high ./...` : 지정한 심각도(severity)와 신뢰도(confidence) 수준으로 검사 level
- `gosec -exclude G101,G304,G401,... ./...` : 지정한 규칙을 검사 대상에서 제외
- `gosec -tests ./...` : 테스트 파일도 함께 검사

## Links
- More information: https://github.com/securego/gosec
