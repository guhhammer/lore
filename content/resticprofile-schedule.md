# resticprofile schedule

Schedule backups and run them in the background.

## Usage
- `resticprofile schedule` : schedule default profile
- `resticprofile --name "group_name" schedule` : schedule a profile (default profile is "default")
- `resticprofile schedule --all` : schedule all profiles
- `resticprofile schedule --no-start` : don't start the job after installing
- `resticprofile status -n profile_name` : display status of scheduled jobs for a profile
- `resticprofile run-schedule "backup@profile_name"` : run a scheduled job manually (used by system scheduler)

## See also
- restic
- resticprofile unschedule

## Categories
system, processes

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/resticprofile-schedule
