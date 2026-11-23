# gitlab-ctl

GitLab 옴니버스를 관리.

## Usage
- `sudo gitlab-ctl status` : 모든 서비스의 상태를 표시
- `sudo gitlab-ctl status nginx` : 특정 서비스의 상태를 표시
- `sudo gitlab-ctl restart` : 모든 서비스 재시작
- `sudo gitlab-ctl restart nginx` : 특정 서비스 재시작
- `sudo gitlab-ctl tail` : 모든 서비스의 로그를 표시 및 `<Ctrl c>`를 누를 때까지 계속 읽기
- `sudo gitlab-ctl tail nginx` : 특정 서비스의 로그를 표시

## Links
- More information: https://docs.gitlab.com/omnibus/maintenance/
