# deborphan

APT 패키지 관리자를 사용하는 운영 체제에서 고아 패키지를 표시합니다.

## Usage
- `deborphan` : 다른 패키지에서 필요로 하지 않는 라이브러리 패키지("libs" 섹션에서) 표시
- `deborphan --guess-all` : "libs" 섹션의 고아 패키지와 라이브러리 이름처럼 보이는 이름을 가진 고아 패키지 나열
- `deborphan -n` : 다른 패키지에서 추천하거나 제안하지만 필수는 아닌 패키지 찾기

## Links
- More information: https://manned.org/deborphan
