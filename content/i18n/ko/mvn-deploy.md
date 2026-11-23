# mvn deploy

아티팩트를 원격 저장소에 배포하는 명령어.

## Usage
- `mvn deploy` : `settings.xml` 파일에 설정된 원격 저장소로 최종 아티팩트 업로드
- `mvn deploy:deploy-file -D url=원격_저장소_url -D repositoryId=settings.xml에_설정된_서버_아이디 -D file=배포될_파일` : maven으로 빌드되지 않은 아티팩트를 원격 저장소에 복사

## Links
- More information: https://manned.org/mvn
