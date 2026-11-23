# clear-history

PowerShell সেশন কমান্ড হিস্ট্রি থেকে এন্ট্রি মুছে ফেলুন।

## Usage
- `Clear-History` : বর্তমান সেশন থেকে সমস্ত কমান্ড হিস্ট্রি পরিষ্কার করুন
- `Clear-History -CommandLine "কমান্ড"` : নির্দিষ্ট নাম দ্বারা কমান্ড পরিষ্কার করুন
- `Clear-History -CommandLine "কমান্ড১", "কমান্ড২", ...` : নাম দ্বারা একাধিক কমান্ড পরিষ্কার করুন
- `Clear-History -Id id_number` : আইডি ব্যবহার করে নির্দিষ্ট হিস্ট্রি এন্ট্রি পরিষ্কার করুন
- `Clear-History -Id id1, id2, ...` : একাধিক আইডি পরিষ্কার করুন
- `Clear-History -Id (start_id..end_id)` : নির্দিষ্ট আইডি রেঞ্জের মধ্যে থাকা কমান্ডগুলো পরিষ্কার করুন
- `Clear-History -WhatIf` : কী মুছে ফেলা হবে সেটা দেখুন
- `Clear-History -Confirm` : পরিষ্কার করার আগে নিশ্চিতকরণ জিজ্ঞাসা করুন
