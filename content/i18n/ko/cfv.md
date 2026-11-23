# cfv

체크섬 파일을 생성하고 검증하는 명령줄 도구.

## Usage
- `cfv -T` : 현재 디렉터리의 체크섬 파일 검증(Test)
- `cfv -Tvnu -f 경로/대상/파일` : 체크섬 파일(file)을 사용해 테스트(Test) 상세 출력(verbosely), 잘못된 파일 이름 변경(rename), 검증되지 않은(unverified) 파일 표시
- `cfv -C -t "sfv|sha256|md5|..." file1 file2 ...` : 선택한 파일들에 대해 특정 타입(type)의 체크섬 파일 생성(Create)
- `cfv -Cr` : 선택된 디렉터리를 재귀적으로(recursively) 순회하며 각각의 체크섬 파일 생성(Create)
- `cfv -C -f sums.sha256 -rr` : 모든 재귀 파일([rr]ecursively)에 대해 하나의 `sha256` 체크섬 파일 생성(Create)
- `cfv -Czvl -p 경로/대상/디렉터리` : `gzipped` 으로 압축된 `.sfv` 파일을 상세 출력(verbosely)과 함께 생성(Create), 심볼릭 링크(`symlinks`)를 따라가고, `경로/대상/디렉터리`를 기본 경로(path)로 설정

## Links
- More information: https://manned.org/cfv
