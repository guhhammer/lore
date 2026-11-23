# fwupdmgr

`fwupd` ஐப் பயன்படுத்தி UEFI உட்பட சாதன நிலைபொருளைப் புதுப்பிப்பதற்கான ஒரு கருவி.

## Usage
- `fwupdmgr get-devices` : `fwupd` மூலம் கண்டறியப்பட்ட அனைத்து சாதனங்களையும் காட்டு
- `fwupdmgr refresh` : LVFS இலிருந்து சமீபத்திய நிலைபொருள் மெட்டாடேட்டாவைப் பதிவிறக்கவும்
- `fwupdmgr get-updates` : உங்கள் கணினியில் உள்ள சாதனங்களுக்குக் கிடைக்கும் புதுப்பிப்புகளை பட்டியலிடுங்கள்
- `fwupdmgr update` : நிலைபொருள் புதுப்பிப்புகளை நிறுவவும்
- `sudo mount -o uid=1000,gid=1000,umask=0022 /dev/sdX /boot` : புதுப்பிப்பு படிக்க-மட்டும் கோப்பு முறைமைபற்றிப் புகார் செய்தால், கூடுதல் சலுகைகளுடன் `/boot` ஐ மீண்டும் ஏற்றவும்
- `fwupdmgr get-history` : நிலைபொருள் புதுப்பிப்பு வரலாற்றைக் காட்டு

## Links
- More information: https://github.com/fwupd/fwupd/blob/main/src/fwupdmgr.md
