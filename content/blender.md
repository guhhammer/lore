# blender

Command-line interface to the Blender 3D computer graphics application. Arguments are executed in the order they are given.

## Usage
- `blender -b path/to/file.blend -a` : render all frames of an animation in the background, without loading the UI (output is saved to `/tmp`)
- `blender -b path/to/file.blend -o //render/frame_###.png -a` : render an animation using a specific image naming pattern, in a path relative (`//`) to the `.blend` file
- `blender -b path/to/file.blend -o /path/to/output_directory -f 10` : render the 10th frame of an animation as a single image, saved to an existing directory (absolute path)
- `blender -b path/to/file.blend -o //path/to/output_directory -f JPEG -f -2` : render the second last frame in an animation as a JPEG image, saved to an existing directory (relative path)
- `blender -b path/to/file.blend -S scene_name -s 10 -e 500 -a` : render the animation of a specific scene, starting at frame 10 and ending at frame 500
- `blender -b path/to/file.blend --python-expr 'import bpy; bpy.data.scenes[0].render.resolution_percentage = 25' -a` : render an animation at a specific resolution, by passing a Python expression
- `blender -b --python-console` : start an interactive Blender session in the terminal with a Python console (do `import bpy` after starting)

## Categories
files, network, text

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/blender
- Wikipedia: https://en.wikipedia.org/wiki/Blender_(software)
