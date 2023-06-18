# CODENAMES-AI

Play Codenames alone! 

![demo.jpg](.thumb/demo.gif)

Codenames is a popular board game designed by Vlaada Chvátil which usually requires teammates to play. In this project, 
a simple transformer-based language model is used so you can play alone.

The user can play as both Spymaster (give clues, AI guesses), or as Agent (AI gives clues, user guesses).

The game mechanica are written in Rust, with the language model being handled by HuggingFace models
in Python. Rust calls the Python interpreter during runtime using `PyO3`.

# How to run

### Linux

Must have Python 3 and Rust+Cargo installed.

```python
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
pip install .

cargo run
```
