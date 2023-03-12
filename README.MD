# CODENAMES-AI

A project to play Codenames alone! Codenames is a popular board game designed by Vlaada 
Chvátil, with an AI based on NLP models.

As of now, the user is only able to play as Spymaster. Support for playing as Agent
is coming in the future.

The game is written in Rust, with the AI being handled by PyTorch-based models
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
