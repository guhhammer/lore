# aa-update-browser

지원되는 인터페이스를 사용하도록 AppArmor 브라우저 프로파일을 업데이트하는 도구. AppArmor 도구 모음의 일부.

## Usage
- `sudo aa-update-browser -l` : 사용 가능한 브라우저 인터페이스 프로파일 목록 출력(list)
- `sudo aa-update-browser -d 경로/대상/프로파일` : 실제 적용 없이 변경 사항 미리 보기(dry-run)
- `sudo aa-update-browser -u 인터페이스1,인터페이스2,... 경로/대상/프로파일` : 특정 인터페이스를 적용하여 프로파일 업데이트(update)
- `sudo aa-update-browser -u '' 경로/대상/프로파일` : 프로파일에서 모든 인터페이스 제거
- `aa-update-browser -h` : 도움말 표시

## Links
- More information: https://manned.org/aa-update-browser
