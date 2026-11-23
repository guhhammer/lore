# qgis_process

Run QGIS processing algorithms from the command line without opening the GUI.

## Usage
- `qgis_process list` : list all available processing algorithms
- `qgis_process help algorithm_id` : display help for a specific algorithm
- `qgis_process run algorithm_id --PARAMETER1=value1 --PARAMETER2=value2 ...` : run a processing algorithm with parameters
- `qgis_process --json run algorithm_id --PARAMETER1=value1 --PARAMETER2=value2 ...` : run an algorithm and output results as JSON
- `qgis_process --skip-loading-plugins run algorithm_id --PARAMETER1=value1 --PARAMETER2=value2 ...` : run without loading plugins for faster startup
- `QT_QPA_PLATFORM=offscreen qgis_process run algorithm_id --PARAMETER1=value1 --PARAMETER2=value2 ...` : run on a headless server without a display
- `qgis_process plugins` : list available and active plugins
- `qgis_process plugins enable|disable plugin_name` : enable or disable an installed plugin

## See also
- qgis

## Categories
text, shell

## Links
- More information: https://docs.qgis.org/latest/en/docs/user_manual/processing/standalone.html
- tldr page: https://tldr.inbrowser.app/pages/common/qgis_process
