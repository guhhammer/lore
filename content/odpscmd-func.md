# odpscmd func

Manage functions in ODPS (Open Data Processing Service).

## Usage
- `list functions;` : [Interactive] Show functions in the current project
- `create function func_name as path.to.package.Func using 'package.jar';` : [Interactive] Create a Java function using a `.jar` resource
- `create function func_name as script.Func using 'script.py';` : [Interactive] Create a Python function using a `.py` resource
- `drop function func_name;` : [Interactive] Delete a function

## Categories
development, system

## Links
- More information: https://www.alibabacloud.com/help/en/maxcompute/user-guide/maxcompute-client
- tldr page: https://tldr.inbrowser.app/pages/common/odpscmd-func
