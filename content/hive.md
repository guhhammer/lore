# hive

Tool for Apache Hive.

## Usage
- `hive` : start a Hive interactive shell
- `hive -e "hiveql_query"` : run HiveQL
- `hive -d key=value -f path/to/file.sql` : run a HiveQL file with a variable substitution
- `hive --hiveconf conf_name=conf_value` : run a HiveQL with HiveConfig (e.g. `mapred.reduce.tasks=32`)

## Categories
files

## Links
- More information: https://cwiki.apache.org/confluence/display/Hive/LanguageManual+Cli
- tldr page: https://tldr.inbrowser.app/pages/common/hive
