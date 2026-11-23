# npm publish

패키지를 npm 레지스트리에서 배포.

## Usage
- `npm publish` : 현재 패키지를 기본 npm 레지스트리에 배포
- `npm publish 경로/대상/패키지_디렉터리` : 특정 디렉터리의 패키지 배포
- `npm publish --access public` : 특정 사용자나 조직(namespace)에 속한 패키지를 공개(public)로 배포
- `npm publish --access restricted` : 특정 사용자나 조직(namespace)에 속한 패키지를 비공개(private) 배포
- `npm publish --registry https://registry.npmjs.org/` : 사용자 정의 레지스트리에 패키지 배포
- `npm publish --dry-run` : 실제 업로드 없이 배포 내용 미리보기
- `npm publish --tag beta` : 특정 배포 태그로 배포 (예: beta)
- `npm publish --otp 일회용_비밀번호` : 2단계 인증(2FA)을 위한 OTP를 사용하여 배포

## Links
- More information: https://docs.npmjs.com/cli/publish/
