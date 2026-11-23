# iscsiadm

Manage iSCSI sessions, nodes, and discovery.

## Usage
- `sudo iscsiadm -m session` : show active iSCSI sessions
- `sudo iscsiadm -m node` : list all known iSCSI nodes
- `sudo iscsiadm -m discovery -t sendtargets -p ip_address` : discover available iSCSI targets on a portal (no authentication)
- `sudo iscsiadm -m node -T iqn -p ip_address:3260 -l` : log in to a specific iSCSI target without authentication
- `sudo iscsiadm -m node -T iqn -p ip_address:3260 -u` : log out from a specific iSCSI target
- `sudo iscsiadm -m node -o new -T iqn -p ip_address:3260` : create an iSCSI node when discovery is blocked (for CHAP authentication)
- `sudo iscsiadm -m node -T iqn -p ip_address:3260 -o update -n node.session.auth.authmethod -v CHAP` : configure CHAP authentication for an iSCSI target

## Categories
users

## Links
- More information: https://manned.org/iscsiadm
- tldr page: https://tldr.inbrowser.app/pages/linux/iscsiadm
