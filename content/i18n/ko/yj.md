# yj

맵 순서를 유지하면서 YAML, TOML, JSON, HCL 형식 간 변환을 수행.

## Usage
- `yj <파일.yml -y` : `stdin`에서 yAML을 읽어 JSON (기본값)으로 변환하고 결과를 `stdout`에 출력
- `yj <파일.toml -ty` : tOML을 yAML로 변환
- `yj <파일.json -jti` : jSON을 tOML로 변환하고 들여쓰기(indentation) 적용
- `yj <파일.hcl -cj` : hcL을 jSON으로 변환
- `yj <파일.yml -ycn` : inf/naN 변환을 무시하고 yAML을 HcL로 변환
- `yj -v` : 버전(version) 정보 표시

## Links
- More information: https://github.com/sclevine/yj
