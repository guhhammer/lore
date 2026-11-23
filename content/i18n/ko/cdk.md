# cdk

AWS Cloud 개발 키트 (CDK)용 CLI.

## Usage
- `cdk ls` : 애플리케이션의 스택 나열
- `cdk synth 스택_이름` : 지정한 스택의 CloudFormation 템플릿을 합성하여 출력
- `cdk deploy 스택_이름1 스택_이름2 ...` : 하나 이상의 스택을 배포
- `cdk destroy 스택_이름1 스택_이름2 ...` : 하나 이상의 스택을 파괴
- `cdk diff 스택_이름` : 지정된 스택을 배포된 스택 또는 로컬 CloudFormation 템플릿과 비교
- `cdk init -l 언어` : 지정된 언어(language)에 대해 현재 디렉터리에 새 CDK 프로젝트를 만듬
- `cdk docs` : 브라우저에서 CDK API 참조를 열기

## Links
- More information: https://docs.aws.amazon.com/cdk/latest/guide/cli.html
