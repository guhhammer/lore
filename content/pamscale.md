# pamscale

Scale a Netpbm image.

## Usage
- `pamscale -wid width -h height path/to/input.pam >path/to/output.pam` : scale an image such that the result has the specified dimensions
- `pamscale -wid width path/to/input.pam >path/to/output.pam` : scale an image such that the result has the specified width, keeping the aspect ratio
- `pamscale -xsc x_factor -ysc y_factor path/to/input.pam >path/to/output.pam` : scale an image such that its width and height is changed by the specified factors
- `pamscale -xyfit bbox_width bbox_height path/to/input.pam >path/to/output.pam` : scale an image such that it fits into the specified bounding box while preserving its aspect ratio
- `pamscale -xyfill box_width box_height path/to/input.pam >path/to/output.pam` : scale an image such that it completely fills the specified box while preserving its aspect ratio

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pamscale.html
- tldr page: https://tldr.inbrowser.app/pages/common/pamscale
