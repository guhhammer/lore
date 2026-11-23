# getuserspns.py

Active Directory ব্যবহারকারীর অ্যাকাউন্টের সাথে সম্পর্কিত Service Principal Names (SPNs) পুনরুদ্ধার করুন। Impacket স্যুইটের অংশ।

## Usage
- `GetUserSPNs.py ডোমেইন/ইউজারনেম:পাসওয়ার্ড -dc-ip ডোমেইন_কন্ট্রোলার_আইপি` : একটি SPN সহ ব্যবহারকারী অ্যাকাউন্ট তালিকাভুক্ত করুন এবং তাদের Kerberos TGS টিকিট অনুরোধ করুন
- `GetUserSPNs.py ডোমেইন/ইউজারনেম -hashes LM_হ্যাশ:NT_হ্যাশ -dc-ip ডোমেইন_কন্ট্রোলার_আইপি` : pass-the-hash অথেন্টিকেশন ব্যবহার করুন
- `GetUserSPNs.py ডোমেইন/ইউজারনেম:পাসওয়ার্ড -dc-ip ডোমেইন_কন্ট্রোলার_আইপি -outputfile আউটপুট_ফাইল/এর/পাথ` : আউটপুট একটি ফাইলে সংরক্ষণ করুন
- `GetUserSPNs.py ডোমেইন/ইউজারনেম:পাসওয়ার্ড -dc-ip ডোমেইন_কন্ট্রোলার_আইপি -request` : শুধুমাত্র TGS টিকিট অনুরোধ করুন
- `GetUserSPNs.py ডোমেইন/ইউজারনেম -dc-ip ডোমেইন_কন্ট্রোলার_আইপি -hashes LM_হ্যাশ:NT_হ্যাশ -request` : pass-the-hash অথেন্টিকেশন ব্যবহার করে শুধুমাত্র TGS টিকিট অনুরোধ করুন

## Links
- More information: https://github.com/fortra/impacket
