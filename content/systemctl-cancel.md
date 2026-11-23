# systemctl cancel

Cancel one or more pending jobs in the system manager or user manager.

## Usage
- `systemctl cancel job_id` : cancel a job by its numeric ID
- `systemctl cancel job_id1 job_id2 ...` : cancel multiple jobs
- `systemctl cancel` : cancel all pending jobs
- `systemctl cancel job_id --user` : cancel a job in the user service manager

## Categories
processes, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-cancel
