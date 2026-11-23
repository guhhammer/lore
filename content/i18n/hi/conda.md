# conda

किसी भी प्रोग्रामिंग भाषा के लिए पैकेज, निर्भरता और परिवेश प्रबंधन। कुछ उप-आदेशों, जैसे कि `create`, के अपने उपयोग प्रलेखन हैं।

## Usage
- `conda create -n परिवेश_नाम python=3.9 matplotlib` : एक नया परिवेश बनाएं और उसमें निर्दिष्ट पैकेज स्थापित करें
- `conda info -e` : सभी परिवेशों की सूची दिखाएं
- `conda activate परिवेश_नाम` : एक परिवेश सक्रिय करें
- `conda deactivate` : एक परिवेश निष्क्रिय करें
- `conda remove -n परिवेश_नाम --all` : एक परिवेश हटाएं (सभी पैकेज हटा दें)
- `conda install python=3.4 numpy` : वर्तमान परिवेश में पैकेज स्थापित करें
- `conda list` : वर्तमान परिवेश में स्थापित पैकेजों की सूची दिखाएं
- `conda clean -a` : अप्रयुक्त पैकेज और कैश हटाएं

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/index.html
