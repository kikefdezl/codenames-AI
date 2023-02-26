from transformers import AutoTokenizer, AutoModelForSequenceClassification
import torch

# Load pre-trained model and tokenizer
model_name = "textattack/bert-base-uncased-MNLI"
tokenizer = AutoTokenizer.from_pretrained(model_name)
model = AutoModelForSequenceClassification.from_pretrained(model_name)

words_a = ['knife', 'blade']
words_b = ['ball', 'sphere']

# Define function to compute semantic similarity
def compute_similarity(word1, word2):
    # Encode the two words as input to the model
    inputs = tokenizer.encode_plus(word1, word2, return_tensors='pt', padding=True, truncation=True, max_length=128)

    # Pass the inputs through the model and get the logits
    outputs = model(**inputs)
    logits = outputs.logits

    # Apply softmax to the logits to get probabilities
    probs = torch.softmax(logits, dim=1)

    # Return the probability that the two words are semantically similar
    return probs[0][1].item()

# Test the function
similarity_score = compute_similarity(*words_a)
print(f'Similarity score between "{words_a[0]}" and "{words_a[1]}":', similarity_score)

similarity_score = compute_similarity(*words_b)
print(f'Similarity score between "{words_b[0]}" and "{words_b[1]}":', similarity_score)

