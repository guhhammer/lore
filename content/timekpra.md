# timekpra

Command-line administration client for Timekpr-nExT, a tool to manage and limit computer usage time per user account.

## Usage
- `timekpra --getuserlist` : list all users known to Timekpr-nExT
- `timekpra --getuserinfo username` : show the stored configuration and time information for a user
- `timekpra --getuserinfort username` : show real-time configuration and time information for a user
- `timekpra --setalloweddays username '1;2;3;4;5'` : set the allowed days of the week for a user (e.g. Monday to Friday)
- `timekpra --settimelimits username '2h;2h;2h;2h;3h'` : set daily time limits for each allowed day
- `timekpra --settimelimitweek username 13h53m20s` : set the weekly time limit for a user
- `timekpra --settimeleft username +|-|= 30m` : add, subtract, or set a user's remaining time for today
- `timekpra --setlockouttype username lock|suspend|suspendwake|terminate|kill|shutdown` : set the lockout action applied once a user's time runs out

## Categories
users, monitoring, text

## Links
- More information: https://mjasnik.gitlab.io/timekpr-next/
- tldr page: https://tldr.inbrowser.app/pages/linux/timekpra
