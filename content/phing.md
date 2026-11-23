# phing

A PHP build tool based on Apache Ant.

## Usage
- `phing` : perform the default task in the `build.xml` file
- `phing -i path/to/build.xml` : initialize a new build file
- `phing task_name` : perform a specific task
- `phing -f path/to/build.xml task_name` : use the given build file path
- `phing -logfile path/to/log_file task_name` : log to the given file
- `phing -Dproperty=value task_name` : use custom properties in the build
- `phing -listener class_name task_name` : specify a custom listener class
- `phing -verbose task_name` : build using verbose output

## Categories
development, files, text

## Links
- More information: https://www.phing.info/guide/chunkhtml/ch03s03.html
- tldr page: https://tldr.inbrowser.app/pages/common/phing
