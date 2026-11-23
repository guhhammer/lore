# dnf config-manager

Fedora ভিত্তিক সিস্টেমে DNF কনফিগারেশন অপশন এবং রিপোজিটরি পরিচালনা করুন। `dnf` এর জন্য ডিফল্ট নয় তবে `dnf-plugins-core` এর মাধ্যমে সমর্থিত।

## Usage
- `dnf config-manager --add-repo=রিপোজিটরি_url` : একটি URL থেকে একটি রিপোজিটরি যোগ করুন (এবং চালু করুন)
- `dnf config-manager --dump` : বর্তমান কনফিগারেশন ভ্যালু প্রিন্ট করুন
- `dnf config-manager --enable রিপোজিটরি_id` : একটি নির্দিষ্ট রিপোজিটরি চালু করুন
- `dnf config-manager --disable রিপোজিটরি_id१ রিপোজিটরি_id२ ...` : নির্দিষ্ট রিপোজিটরিগুলি নিষ্ক্রিয় করুন
- `dnf config-manager --setopt=অপশন=ভ্যালু` : একটি রিপোজিটরির জন্য একটি কনফিগারেশন অপশন সেট করুন
- `dnf config-manager --help-cmd` : সাহায্য প্রদর্শন

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/config_manager.html
