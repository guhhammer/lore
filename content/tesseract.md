# tesseract

OCR (Optical Character Recognition) engine.

## Usage
- `tesseract path/to/image.png path/to/output_file` : recognize text in an image and save it to the given path (a `.txt` extension is added automatically)
- `tesseract -l deu path/to/image.png path/to/output_file` : specify a custom language (default is English) with an ISO 639-2 code (e.g. deu = Deutsch = German)
- `tesseract --list-langs` : list the ISO 639-2 codes of installed languages
- `tesseract --psm 0..13 path/to/image.png path/to/output_file` : specify a custom page segmentation mode (default is 3)
- `tesseract --help-psm` : list page segmentation modes and their descriptions

## Categories
files, containers, media

## Links
- More information: https://github.com/tesseract-ocr/tesseract/blob/main/doc/tesseract.1.asc
- tldr page: https://tldr.inbrowser.app/pages/common/tesseract
- Wikipedia: https://en.wikipedia.org/wiki/Tesseract_(software)
