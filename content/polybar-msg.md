# polybar-msg

Control `polybar` using inter-process-messaging (IPC). Note: IPC is disabled by default and can be enabled by setting `enable-ipc = true` in the Polybar config.

## Usage
- `polybar-msg cmd quit` : quit the bar
- `polybar-msg cmd restart` : restart the bar in-place
- `polybar-msg cmd hide` : hide the bar (does nothing if the bar is already hidden)
- `polybar-msg cmd show` : show the bar again (does nothing if the bar is not hidden)
- `polybar-msg cmd toggle` : toggle between hidden/visible
- `polybar-msg action "#module_name.action_name.data_string"` : execute a module action (the data string is optional)
- `polybar-msg -p process_id cmd|action payload` : only send messages to a specific Polybar instance (all instances by default)

## Categories
processes

## Links
- More information: https://polybar.readthedocs.io/en/stable/user/ipc.html
- tldr page: https://tldr.inbrowser.app/pages/common/polybar-msg
