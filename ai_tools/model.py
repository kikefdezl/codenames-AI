from __future__ import annotations

import numpy as np
import torch
import torch.nn.functional as F
from sklearn.metrics.pairwise import cosine_similarity
from transformers import AutoTokenizer, AutoModel


# Mean Pooling - Take attention mask into account for correct averaging
def mean_pooling(model_output, attention_mask):
    token_embeddings = model_output[0]  # First element of model_output contains all token embeddings
    input_mask_expanded = attention_mask.unsqueeze(-1).expand(token_embeddings.size()).float()
    return torch.sum(token_embeddings * input_mask_expanded, 1) / torch.clamp(input_mask_expanded.sum(1), min=1e-9)


def get_model():
    tokenizer = AutoTokenizer.from_pretrained('sentence-transformers/all-MiniLM-L12-v1')
    model = AutoModel.from_pretrained('sentence-transformers/all-MiniLM-L12-v1')

    return tokenizer, model


def compute_words_to_words_similarity(words_a: str | list[str], words_b: str | list[str]) -> np.ndarray:
    if isinstance(words_a, str):
        words_a = [words_a]
    if isinstance(words_b, str):
        words_b = [words_b]
    tokenizer, model = get_model()

    encoded_a = tokenizer(words_a, padding=True, truncation=True, return_tensors='pt')
    encoded_b = tokenizer(words_b, padding=True, truncation=True, return_tensors='pt')

    with torch.no_grad():
        model_output_a = model(**encoded_a)
        model_output_b = model(**encoded_b)

    embeddings_a = mean_pooling(model_output_a, encoded_a['attention_mask'])
    embeddings_b = mean_pooling(model_output_b, encoded_b['attention_mask'])

    embeddings_a = F.normalize(embeddings_a, p=2, dim=1)
    embeddings_b = F.normalize(embeddings_b, p=2, dim=1)

    similarities = cosine_similarity(embeddings_a, embeddings_b)
    return similarities


if __name__ == "__main__":
    words_a = ["BANANA", "AIRPLANE", "EGYPT", "SUITCASE", "WHEEL"]
    words_b = ["APPLE", "CAR", "GOVERNMENT", "RISK", "STONE", "METAL"]
    similarities = compute_words_to_words_similarity(words_a, words_b)
    for a, word_a in enumerate(words_a):
        for b, word_b in enumerate(words_b):
            print(f"Similarity {word_a} - {word_b}: {similarities[a][b]}")
