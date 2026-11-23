# choco source

சாக்லேட்டி மூலம் தொகுப்புகளுக்கான ஆதாரங்களை நிர்வகிக்கவும்.

## Usage
- `choco source list` : தற்போது கிடைக்கக்கூடிய ஆதாரங்களை பட்டியலிடுங்கள்
- `choco source add -n பெயர் -s முகவரி` : புதிய தொகுப்பு மூலத்தைச் சேர்க்கவும்
- `choco source add -n பெயர் -s முகவரி -u பயனர்ப்பெயர் -p கடவுச்சொல்` : நற்சான்றிதழ்களுடன் புதிய தொகுப்பு மூலத்தைச் சேர்க்கவும்
- `choco source add -n பெயர் -s முகவரி --cert சான்றிதழ்\பாதை` : கிளையன்ட் சான்றிதழுடன் புதிய தொகுப்பு மூலத்தைச் சேர்க்கவும்
- `choco source enable -n பெயர்` : தொகுப்பு மூலத்தை இயக்கு
- `choco source disable -n பெயர்` : ஒரு தொகுப்பு மூலத்தை முடக்கு
- `choco source remove -n பெயர்` : தொகுப்பு மூலத்தை அகற்றவும்

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/source/
