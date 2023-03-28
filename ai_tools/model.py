from __future__ import annotations

from sentence_transformers import SentenceTransformer
from sklearn.metrics.pairwise import cosine_similarity

model = SentenceTransformer('bert-base-nli-mean-tokens')


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
    word_a_vec = model.encode(reference_word, batch_size=64).reshape(1, -1)
    word_b_vecs = model.encode(words, batch_size=64)
    similarity = cosine_similarity(word_a_vec, word_b_vecs)[0]
    return similarity
