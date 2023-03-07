from __future__ import annotations

from transformers import AutoTokenizer, AutoModelForSequenceClassification
import numpy as np
import torch
from tqdm import tqdm

# Load pre-trained model and tokenizer
model_name = "textattack/bert-base-uncased-MNLI"
tokenizer = AutoTokenizer.from_pretrained(model_name)
model = AutoModelForSequenceClassification.from_pretrained(model_name)


# Define function to compute semantic similarity
def compute_word_to_words_similarity(
        reference_word: str,
        words: list[str], 
) -> list[float]:
    """
    Computes the semantic similarity between a reference word and
    a list of other words.
    :arg reference_word: The reference word to compute similarity with.
    :arg words: The list of words to compare the reference word with.
    :return: A list of float values, pertaining to the similarity between
        the reference word and the list of other words. The returned values 
        are in the same order as the input list.
    """
    print(reference_word)
    print()
    print(words)
    result = []
    for word in words:
        # Encode the two words as input to the model
        inputs = tokenizer.encode_plus(
            reference_word,
            word, 
            return_tensors='pt', 
            padding=True, 
            truncation=True, 
            max_length=128
        )

        # Pass the inputs through the model and get the logits
        outputs = model(**inputs)
        logits = outputs.logits

        # Apply softmax to the logits to get probabilities
        probs = torch.softmax(logits, dim=1)

        # Return the probability that the two words are semantically similar
        result.append(probs[0][1].item())
    return result


def get_similarity_matrix(wordlist: list[str]) -> np.ndarray:
    n_words = len(wordlist)
    similarity_matrix = np.zeros((n_words, n_words), dtype=np.float32)
    for i in tqdm(list(range(n_words)), "Computing similarity matrix"):
        word_a = wordlist[i]
        for j in range(i, n_words):
            word_b = wordlist[j]
            similarity = compute_similarity(word_a, word_b)

            if i == j:
                continue

            similarity_matrix[i, j] = similarity
            similarity_matrix[j, i] = similarity
    return similarity_matrix


def plot_similarity_matrix(matrix: np.ndarray, wordlist: list[str]):
    fig, ax = plt.subplots()

    ax.matshow(matrix, cmap=plt.cm.Oranges)
    plt.yticks(range(len(wordlist)))
    plt.xticks(range(len(wordlist)))
    ax.set_xticklabels(wordlist, rotation=90)
    ax.set_yticklabels(wordlist)
