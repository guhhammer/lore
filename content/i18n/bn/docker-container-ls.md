# docker container ls

Docker কন্টেইনারগুলো তালিকাভুক্ত করুন।

## Usage
- `docker ps` : বর্তমানে চলমান Docker কন্টেইনারগুলো তালিকাভুক্ত করুন
- `docker ps -a` : সমস্ত Docker কন্টেইনার তালিকাভুক্ত করুন (চলমান এবং বন্ধ)
- `docker ps -l` : সর্বশেষ তৈরি হওয়া কন্টেইনার দেখান (সব অবস্থা অন্তর্ভুক্ত)
- `docker ps -f "name=name"` : নামে একটি সাবস্ট্রিং থাকা কন্টেইনারগুলো ফিল্টার করুন
- `docker ps -f "ancestor=image:tag"` : নির্দিষ্ট একটি ইমেজকে পূর্বসূরি হিসেবে থাকা কন্টেইনারগুলো ফিল্টার করুন
- `docker ps -f "exited=code" -a` : প্রস্থান (exit) স্ট্যাটাস কোড অনুযায়ী কন্টেইনারগুলো ফিল্টার করুন
- `docker ps -f "status=status"` : অবস্থা অনুযায়ী কন্টেইনারগুলো ফিল্টার করুন (created, running, removing, paused, exited, এবং dead)
- `docker ps -f "volume=ডিরেক্টরি/এর/পাথ" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : নির্দিষ্ট একটি ভলিউম মাউন্ট করা বা নির্দিষ্ট পথে ভলিউম মাউন্ট থাকা কন্টেইনারগুলো ফিল্টার করুন

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/ls/
