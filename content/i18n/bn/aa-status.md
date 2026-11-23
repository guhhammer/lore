# aa-status

বর্তমানে লোড করা AppArmor মডিউলগুলোর তালিকা দেখুন।

## Usage
- `sudo aa-status` : স্ট্যাটাস চেক করুন
- `sudo aa-status --json` : JSON ফরম্যাটে স্ট্যাটাস প্রদর্শন করুন
- `sudo aa-status --pretty-json` : সুন্দর JSON ফরম্যাটে স্ট্যাটাস প্রদর্শন করুন
- `sudo aa-status --profiled` : লোড করা পলিসিগুলোর সংখ্যা প্রদর্শন করুন
- `sudo aa-status --enforced` : লোড করা enforcing পলিসিগুলোর সংখ্যা প্রদর্শন করুন
- `sudo aa-status --complaining` : লোড করা non-enforcing পলিসিগুলোর সংখ্যা প্রদর্শন করুন
- `sudo aa-status --kill` : লোড করা enforcing পলিসিগুলোর সংখ্যা প্রদর্শন করুন যা টাস্ক kill করে

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-status.8
