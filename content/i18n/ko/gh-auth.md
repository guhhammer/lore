# gh auth

GitHub 호스트에 인증.

## Usage
- `gh auth login` : 대화형 프롬프트로 로그인
- `echo your_token | gh auth login --with-token` : `stdin`에서 토큰으로 로그인 (<https://github.com/settings/tokens>에서 생성됨)
- `gh auth status` : 로그인 여부 확인
- `gh auth logout` : 로그아웃
- `gh auth login -h github.example.com` : 특정 GitHub 엔터프라이즈 서버로 로그인
- `gh auth refresh` : 인증 자격 증명이 올바른 최소 범위를 갖도록 세션 새로 고침 (이전에 요청된 추가 범위 제거)
- `gh auth refresh -s repo,admin:repo_hook,admin:org,admin:public_key,admin:org_hook,...` : 권한 범위 확장

## Links
- More information: https://cli.github.com/manual/gh_auth
