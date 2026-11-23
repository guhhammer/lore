# oc

OpenShift 컨테이너 플랫폼 CLI. 애플리케이션 및 컨테이너 관리를 허용합니다.

## Usage
- `oc login` : openShift 컨테이너 플랫폼 서버에 로그인
- `oc new-project 프로젝트_이름` : 새 프로젝트 생성
- `oc project 프로젝트_이름` : 기존 프로젝트로 전환
- `oc new-app 저장소_url --name 애플리케이션` : 프로젝트에 새 애플리케이션 추가
- `oc rsh 포드_이름` : 컨테이너에 원격 셸 세션 열기
- `oc get pods` : 프로젝트 내 포드 나열
- `oc logout` : 현재 세션에서 로그아웃
