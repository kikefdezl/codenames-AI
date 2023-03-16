from __future__ import annotations

import gensim

from ai_tools.download import download_model
from ai_tools.settings import MODEL_NAME, FULL_MODEL_PATH


def compute_word_to_word_similarity(model, word_a: str, word_b: str) -> float:
    # Encode the two words as input to the model
    try:
        similarity = model.similarity(word_a, word_b)
    except KeyError:
        similarity = 0.0
    return similarity


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
    download_model(MODEL_NAME)
    model = gensim.models.KeyedVectors.load_word2vec_format(
        FULL_MODEL_PATH.with_suffix(".bin"),
        binary=True
    )
    result = []
    for word in words:
        result.append(compute_word_to_word_similarity(model, reference_word, word))
    return result