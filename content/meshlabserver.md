# meshlabserver

Interface for the MeshLab 3D mesh processing software.

## Usage
- `meshlabserver -i input.stl -o output.obj` : convert an STL file to an OBJ file
- `meshlabserver -i input.wrl -o output.off -om vn fn` : convert a WRL file to an OFF file, including the vertex and face normals in the output mesh
- `meshlabserver -d path/to/file` : dump a list of all the available processing filters into a file
- `meshlabserver -i input.ply -o output.ply -s filter_script.mlx` : process a 3D file using a filter script created in the MeshLab GUI (Filters > Show current filter script > Save Script)
- `meshlabserver -i input.x3d -o output.x3d -s filter_script.mlx -l logfile` : process a 3D file using a filter script, writing the output of the filters into a log file

## Categories
files, network, development

## Links
- More information: https://manned.org/meshlabserver
- tldr page: https://tldr.inbrowser.app/pages/common/meshlabserver
