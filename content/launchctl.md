# launchctl

Control Apple's `launchd` manager for launch daemons (system-wide services) and launch agents (per-user programs). `launchd` loads XML-based `*.plist` files placed in the appropriate locations, and runs the corresponding commands according to their defined schedule.

## Usage
- `launchctl load ~/Library/LaunchAgents/my_script.plist` : activate a user-specific agent to be loaded into `launchd` whenever the user logs in
- `sudo launchctl load /Library/LaunchAgents/root_script.plist` : activate an agent which requires root privileges to run and/or should be loaded whenever any user logs in (note the absence of `~` in the path)
- `sudo launchctl load /Library/LaunchDaemons/system_daemon.plist` : activate a system-wide daemon to be loaded whenever the system boots up (even if no user logs in)
- `launchctl list` : show all loaded agents/daemons, with the PID if the process they specify is currently running, and the exit code returned the last time they ran
- `launchctl unload ~/Library/LaunchAgents/my_script.plist` : unload a currently loaded agent, e.g. to make changes (Note: The plist file is automatically loaded into `launchd` after a reboot and/or logging in)
- `launchctl start script_file` : manually run a known (loaded) agent/daemon, even if it is not the right time (Note: This command uses the agent's label, rather than the filename)
- `launchctl stop script_file` : manually kill the process associated with a known agent/daemon, if it is running

## Categories
system, users, development, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/launchctl.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/launchctl
