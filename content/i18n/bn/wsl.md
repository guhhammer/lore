# wsl

Windows Subsystem for Linux পরিচালনা করুন।

## Usage
- `wsl shell_command` : একটি লিনাক্স shell শুরু করুন (ডিফল্ট ডিস্ট্রিবিউশন এ)
- `wsl -e কমান্ড command_arguments` : একটি shell ব্যবহার না করে একটি লিনাক্স কমান্ড চালান
- `wsl -d distribution shell_command` : একটি নির্দিষ্ট ডিস্ট্রিবিউশন উল্লেখ করুন
- `wsl -l` : ব্যবহারযোগ্য ডিস্ট্রিবিউশনগুলি তালিকা করুন
- `wsl --export distribution ডিস্ট্রো_ফাইল.tar/এর/পাথ` : একটি ডিস্ট্রিবিউশন `.tar` ফাইলে এক্সপোর্ট করুন
- `wsl --import distribution ইনস্টল_লোকেশন/এর/পাথ ডিস্ট্রো_ফাইল.tar/এর/পাথ` : একটি `.tar` ফাইল থেকে ডিস্ট্রিবিউশন ইম্পোর্ট করুন
- `wsl --set-version distribution version` : নির্দিষ্ট ডিস্ট্রিবিউশনের জন্য ব্যবহৃত wsl এর সংস্করণ পরিবর্তন করুন
- `wsl --shutdown` : windows Subsystem for Linux বন্ধ করুন

## Links
- More information: https://learn.microsoft.com/windows/wsl/reference
