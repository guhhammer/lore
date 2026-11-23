# odpscmd table

Create and modify tables in ODPS (Open Data Processing Service).

## Usage
- `create table table_name (col type) partitioned by (col type) lifecycle days;` : [Interactive] Create a table with partition and lifecycle
- `create table table_name like another_table;` : [Interactive] Create a table based on the definition of another table
- `alter table table_name add partition (partition_spec);` : [Interactive] Add partition to a table
- `alter table table_name drop partition (partition_spec);` : [Interactive] Delete partition from a table
- `drop table table_name;` : [Interactive] Delete table

## Categories
databases, disks, system

## Links
- More information: https://www.alibabacloud.com/help/en/maxcompute/user-guide/maxcompute-client
- tldr page: https://tldr.inbrowser.app/pages/common/odpscmd-table
