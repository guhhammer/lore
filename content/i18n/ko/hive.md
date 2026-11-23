# hive

Apache Hive용 CLI 도구.

## Usage
- `hive` : hive 대화형 쉘을 시작
- `hive -e "hiveql_쿼리"` : hiveQL 실행
- `hive -d 키=값 -f 경로/대상/파일.sql` : 변수 대체를 사용해서 HiveQL 파일을 실행
- `hive --hiveconf 구성_이름=구성_값` : hiveConfig를 사용해 HiveQL을 실행 (예: `mapred.reduce.tasks=32`)

## Links
- More information: https://cwiki.apache.org/confluence/display/Hive/LanguageManual+Cli
