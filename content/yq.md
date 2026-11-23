# yq

A lightweight and portable YAML processor.

## Usage
- `yq eval path/to/file.yaml` : output a YAML file, in pretty-print format (v4+)
- `yq read path/to/file.yaml -C` : output a YAML file, in pretty-print format (v3)
- `yq eval '.[0]' path/to/file.yaml` : output the first element in a YAML file that contains only an array (v4+)
- `yq read path/to/file.yaml '[0]'` : output the first element in a YAML file that contains only an array (v3)
- `yq eval '.key = "value"' -i path/to/file.yaml` : set (or overwrite) a key to a value in a file (v4+)
- `yq write -i path/to/file.yaml 'key' 'value'` : set (or overwrite) a key to a value in a file (v3)
- `yq eval-all 'select(filename == "path/to/file1.yaml") * select(filename == "path/to/file2.yaml")' path/to/file1.yaml path/to/file2.yaml` : merge two files and print to `stdout` (v4+)
- `yq merge path/to/file1.yaml path/to/file2.yaml -C` : merge two files and print to `stdout` (v3)

## Categories
files, text, documents

## Links
- More information: https://mikefarah.gitbook.io/yq/
- tldr page: https://tldr.inbrowser.app/pages/common/yq
