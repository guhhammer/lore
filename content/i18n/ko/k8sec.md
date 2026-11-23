# k8sec

Kubernetes 시크릿 관리 도구.

## Usage
- `k8sec list` : 모든 시크릿 나열
- `k8sec list 비밀_이름 --base64` : 특정 시크릿을 Base64로 인코딩된 문자열로 나열
- `k8sec set 비밀_이름 key=값` : 시크릿 값 설정
- `k8sec set --base64 비밀_이름 key=인코딩된_값` : base64로 인코딩된 값 설정
- `k8sec unset 비밀_이름` : 시크릿 해제
- `k8sec load -f 경로/대상/파일 비밀_이름` : 파일에서 시크릿 불러오기
- `k8sec dump -f 경로/대상/파일 비밀_이름` : 파일로 시크릿 덤프

## Links
- More information: https://github.com/dtan4/k8sec
