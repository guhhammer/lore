# hydra

Online password guessing tool. Protocols supported include FTP, HTTP(S), SMTP, SNMP, XMPP, SSH, and more.

## Usage
- `hydra-wizard` : start Hydra's wizard
- `hydra -l username -P path/to/wordlist.txt host_ip ssh` : guess SSH credentials using a given username and a list of passwords
- `hydra -L path/to/usernames.txt -P path/to/wordlist.txt host_ip https-post-form "url_without_host:https_post_request:login_failed_string"` : guess HTTPS webform credentials using two specific lists of usernames and passwords ("https_post_request" can be like "username=^USER^&password=^PASS^")
- `hydra -L path/to/usernames.txt -P path/to/wordlist.txt -t n_tasks host_ip ftp` : guess FTP credentials using usernames and passwords lists, specifying the number of threads
- `hydra -l username -P path/to/wordlist.txt -f host_ip mysql` : guess MySQL credentials using a username and a passwords list, exiting when a username/password pair is found
- `hydra -l username -P path/to/wordlist.txt -V rdp://host_ip` : guess RDP credentials using a username and a passwords list, showing each attempt
- `hydra -C path/to/username_password_pairs.txt imap://host_range_cidr` : guess IMAP credentials on a range of hosts using a list of colon-separated username/password pairs
- `hydra -L path/to/usernames.txt -P path/to/wordlist.txt -M path/to/hosts.txt -F pop3` : guess POP3 credentials on a list of hosts using usernames and passwords lists, exiting when a username/password pair is found

## Categories
network, files, users

## Links
- More information: https://manned.org/hydra
- tldr page: https://tldr.inbrowser.app/pages/common/hydra
- Wikipedia: https://en.wikipedia.org/wiki/Hydra_(software)
