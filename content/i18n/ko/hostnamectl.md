# hostnamectl

컴퓨터의 호스트명을 가져오거나 설정합니다.

## Usage
- `hostnamectl` : 컴퓨터의 호스트명 가져오기
- `sudo hostnamectl set-hostname "호스트명"` : 컴퓨터의 호스트명 설정
- `sudo hostnamectl set-hostname --static "호스트명.example.com" && sudo hostnamectl set-hostname --pretty "호스트명"` : 컴퓨터에 보기 좋은 호스트명 설정
- `sudo hostnamectl set-hostname --pretty ""` : 호스트명을 기본값으로 재설정

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/hostnamectl.html
