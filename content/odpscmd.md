# odpscmd

Aliyun ODPS (Open Data Processing Service) command-line tool. Some subcommands such as `inst` have their own usage documentation.

## Usage
- `odpscmd --config=odps_config.ini` : start the command-line with a custom configuration file
- `use project_name;` : [Interactive] Switch current project
- `show tables;` : [Interactive] Show tables in the current project
- `desc table_name;` : [Interactive] Describe a table
- `show partitions table_name;` : [Interactive] Show table partitions
- `desc table_name partition (partition_spec);` : [Interactive] Describe a partition

## Categories
text, documents, monitoring

## Links
- More information: https://www.alibabacloud.com/help/en/maxcompute/user-guide/maxcompute-client
- tldr page: https://tldr.inbrowser.app/pages/common/odpscmd
