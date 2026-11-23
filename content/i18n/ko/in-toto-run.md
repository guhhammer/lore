# in-toto-run

공급망 단계 수행과 동시에 링크 메타데이터를 생성하는 도구.

## Usage
- `in-toto-run -n 태그 -p . --signing-key 키_파일 -- git tag v1.0` : git 저장소에 태그를 생성하고 링크 파일에 서명
- `in-toto-run -n package -m project -p project.tar.gz -- tar czf project.tar.gz project` : tarball을 생성하고, 원본 파일은 materials로, 생성된 tarball은 product로 기록
- `in-toto-run -n review --signing-key 키_파일 -m 문서.pdf -x` : 리뷰 작업에 대한 서명된 증명 생성
- `in-toto-run -n scan --signing-key 키_파일 -p report.json -- /bin/sh -c "trivy --output report.json --format json 경로/대상/이미지"` : trivy로 이미지를 스캔하고 링크 파일 생성

## Links
- More information: https://in-toto.readthedocs.io/en/latest/command-line-tools/in-toto-run.html
