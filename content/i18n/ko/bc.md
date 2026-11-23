# bc

임의의 정밀 계산기 언어.

## Usage
- `bc` : 대화형 세션을 시작
- `bc -i -l` : 표준 수학 라이브러리(library)가 활성화된 상태에서 대화형(interactive) 세션을 시작
- `echo '5 / 3' | bc` : 표현식을 계산
- `bc 경로/대상/스크립트.bc` : 스크립트 실행
- `echo 'scale = 10; 5 / 3' | bc` : 지정된 척도를 사용해 표현식을 계산
- `echo 's|c|a|l|e(1)' | bc -l` : `mathlib`을 사용하여 사인/코사인/아크탄젠트/자연 로그/지수 함수를 계산
- `echo "define factorial(n) { if (n <= 1) return 1; return n*factorial(n-1); }; factorial(10)" | bc` : 인라인 계승 스크립트를 실행

## Links
- More information: https://manned.org/bc
