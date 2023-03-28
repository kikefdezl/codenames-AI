from pathlib import Path

from ai_tools.model import compute_word_to_words_similarity

wordfile = Path("../data/google-10000-english-filtered.txt")


def main():
    with open(wordfile, 'r') as f:
        wordlist = f.read().split('\n')

    for word in wordlist:
        result = compute_word_to_words_similarity(word, wordlist)
        print(result)


if __name__ == "__main__":
    main()
