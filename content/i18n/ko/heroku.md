# heroku

Heroku 애플리케이션 생성 및 관리.

## Usage
- `heroku login` : heroku 계정에 로그인
- `heroku create` : heroku 애플리케이션 생성
- `heroku logs --app 애플리케이션_이름` : 애플리케이션 로그 표시
- `heroku run 프로세스_이름 --app 애플리케이션_이름` : dyno (Heroku 가상 머신) 내에서 일회성 프로세스를 실행
- `heroku ps --app 애플리케이션_이름` : 애플리케이션의 dynos (Heroku 가상 머신) 나열
- `heroku destroy --app 애플리케이션_이름` : 애플리케이션 영구적으로 삭제
