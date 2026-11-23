# pm2

Process manager for Node.js. Used for log management, monitoring, and configuring processes.

## Usage
- `pm2 start app.js --name application_name` : start a process with a name that can be used for later operations
- `pm2 list` : list processes
- `pm2 monit` : monitor all processes
- `pm2 stop application_name` : stop a process
- `pm2 restart application_name` : restart a process
- `pm2 save` : dump all processes for resurrecting them later
- `pm2 resurrect` : resurrect previously dumped processes

## Categories
processes, monitoring

## Links
- More information: https://pm2.keymetrics.io/docs/usage/quick-start/
- tldr page: https://tldr.inbrowser.app/pages/common/pm2
