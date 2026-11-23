# manim

用于制作数学动画的视频引擎。

## Usage
- `manim 路径/文件.py 场景名称` : 使用默认设置渲染 Python 脚本中的场景
- `manim -pql 路径/文件.py 场景名称` : 实时预览渲染（渲染后自动打开视频）
- `manim -pqh 路径/文件.py 场景名称` : 以高质量渲染（1080p 60 帧）
- `manim -o 输出文件名 路径/文件.py 场景名称` : 指定自定义输出文件名
- `manim -r 1920,1080 --fps 60 路径/文件.py 场景名称` : 使用指定分辨率和帧率进行渲染
- `manim --list_scenes 路径/文件.py` : 列出文件中可用的场景而不进行渲染
- `manim --help` : 显示帮助

## Links
- More information: https://docs.manim.community/en/stable/tutorials/quickstart.html
