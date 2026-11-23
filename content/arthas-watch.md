# arthas-watch

Method invoke data observation.

## Usage
- `watch class_pattern method_pattern '{ params[0],returnObj }' -x 4` : observe the first parameter and return value of method, and expand the nested attributes of the object to 4 levels
- `watch class_pattern method_pattern '{ params[1],returnObj }' '"5".equals(params[0])' -x 4` : when the value of the first parameter of the method is 5, output the second parameter and return value, and expand the object 4 layers
- `watch class_pattern method_pattern '{ params[1].count }' -e -s` : when the method returns or an exception occurs, observe the count property of the second parameter

## See also
- arthas
- arthas-trace

## Links
- More information: https://arthas.aliyun.com/en/doc/watch.html
- tldr page: https://tldr.inbrowser.app/pages/common/arthas-watch
