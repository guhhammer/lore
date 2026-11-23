# cam

Frontend tool for `libcamera`.

## Usage
- `cam -l` : list available cameras
- `cam -c camera_index --list-controls` : list controls of a camera
- `cam -c camera_index -Cframes_to_capture -F` : write frames to a folder
- `cam -c camera_index -C -S` : display camera feed in a window

## See also
- v4l2-ctl

## Links
- More information: https://docs.libcamera.org/
- tldr page: https://tldr.inbrowser.app/pages/linux/cam
