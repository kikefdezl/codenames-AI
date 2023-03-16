from pathlib import Path

MODEL_DICT = {
    "wikipedia2vec_100b": {
        "http_link": "http://wikipedia2vec.s3.amazonaws.com/models/en/2018-04-20/enwiki_20180420_100d.pkl.bz2"
    },
    "googlenews2vec": {
        "gdrive_link": "https://drive.google.com/u/1/uc?id=0B7XkCwpI5KDYNlNUTTlSS21pQmM&export=download"
    }
}

MODEL_NAME = "googlenews2vec"
MODEL_DIRECTORY = "model"

FULL_MODEL_PATH = (Path(__file__).parent.parent.resolve()
                   / MODEL_DIRECTORY
                   / MODEL_NAME)

