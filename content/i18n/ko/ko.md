# ko

Kubernetes에서 Go 애플리케이션을 컨테이너 이미지로 빌드하고 배포.

## Usage
- `ko build import_경로` : go 패키지 import 경로로부터 컨테이너 이미지 빌드 및 게시
- `ko build -L import_경로` : 컨테이너 이미지를 빌드해, 로컬 Docker 데몬에 로드
- `ko apply -f 경로/대상/매니페스트.yaml` : go 이미지 참조를 digest로 변환하여 Kubernetes 매니페스트 적용
- `ko create -f 경로/대상/매니페스트.yaml` : 이미지 참조가 변환된 Kubernetes 리소스 생성
- `ko resolve -f 경로/대상/매니페스트.yaml` : kubernetes 매니페스트를 적용하지 않고 변환 결과만 출력
- `ko run import_경로` : go 패키지를 빌드해서 Kubernetes에서 실행
- `ko delete -f 경로/대상/매니페스트.yaml` : 매니페스트에 정의된 Kubernetes 리소스 삭제
- `ko login registry.example.com -u 사용자명 -p 비밀번호` : 컨테이너 레지스트리에 로그인

## Links
- More information: https://ko.build/reference/ko/
