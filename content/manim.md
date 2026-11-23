# manim

Animation engine for explanatory math videos.

## Usage
- `manim path/to/file.py SceneName` : render a scene from a Python script using the default settings
- `manim -pql path/to/file.py SceneName` : render with live preview (auto-opens the video file after rendering)
- `manim -pqh path/to/file.py SceneName` : render at high quality (1080p 60fps)
- `manim -o output_file_name path/to/file.py SceneName` : specify a custom output file name
- `manim -r 1920,1080 --fps 60 path/to/file.py SceneName` : render using a specific resolution and frame rate
- `manim --list_scenes path/to/file.py` : list available scenes in a file without rendering
- `manim --help` : display help

## Categories
files

## Links
- More information: https://docs.manim.community/en/stable/tutorials/quickstart.html
- tldr page: https://tldr.inbrowser.app/pages/common/manim
