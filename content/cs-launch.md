# cs launch

Launch an application from the name directly from Maven dependencies without the need of installing it.

## Usage
- `cs launch application_name -- argument1 argument2 ...` : launch a specific application with arguments
- `cs launch application_name:application_version -- argument1 argument2 ...` : launch a specific application version with arguments
- `cs launch group_id:artifact_id:artifact_version --main-class path/to/main_class_file` : launch a specific version of an application specifying which is the main file
- `cs launch --java-opt -Doption_name1:option_value1 -Doption_name2:option_value2 ... --java-opt -Xjvm_option1 -Xjvm_option2 ... application_name` : launch an application with specific Java options and JVM memory ones

## Categories
packages

## Links
- More information: https://get-coursier.io/docs/cli-launch
- tldr page: https://tldr.inbrowser.app/pages/common/cs-launch
