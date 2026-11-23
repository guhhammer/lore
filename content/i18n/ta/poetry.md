# poetry

Python தொகுதிகள் மற்றும் சார்புகளை நிர்வகிக்க. `about`, `check`, `env`, etc. போன்ற சிலச் சார்கட்டளைகளுக்குத் தனித்தனி பயன்பாட்டு ஆவணங்கள் உள்ளன.

## Usage
- `poetry new தோட்டத்தின்_பெயர்` : குறிப்பிட்ட பெயருடன் புதிய Poetry திட்டத்தை அடைவில் உருவாக்கு
- `poetry add சார்பு` : ஒரு சார்பையும் அதன் துணை சார்புகளையும் நிறுவி `pyproject.toml` கோப்பில் சேர்க்க
- `poetry install` : தற்போதைய அடைவில் உள்ள `pyproject.toml` கோப்பைப் பயன்படுத்தி திட்ட சார்புகளை நிறுவு
- `poetry init` : தற்போதைய அடைவை புதிய Poetry திட்டமாக ஊடாடும் வகையில் (அல்லது `-n` சேர்த்து ஊடாடதாக) தொடங்கு
- `poetry update` : அனைத்து சார்புகளின் சமீபத்திய பதிப்பைப் பெற்று `poetry.lock` கோப்பை புதுப்பிக்க
- `poetry run கட்டளை` : திட்டத்தின் மெய்நிகர் சூழலில் ஒரு கட்டளையை இயக்கு
- `poetry version patch|minor|major|prepatch|preminor|premajor|prerelease` : திட்டத்தின் பதிப்பை `pyproject.toml` கோப்பில் மாற்று
- `eval "$(poetry env activate)"` : திட்டத்தின் மெய்நிகர் சூழலில் ஒரு shell திறக்க (2.0 க்குக் கீழே உள்ள பதிப்புகளுக்கு `poetry shell` பயன்படுத்தவும்)

## Links
- More information: https://python-poetry.org/docs/cli/
