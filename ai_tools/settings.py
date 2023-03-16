from pathlib import Path

MODEL_DICT = {
    "wikipedia2vec_100b": {
        "file_name": "wikipedia2vec_100b.bin",
        "http_link": "http://wikipedia2vec.s3.amazonaws.com/models/en/2018-04-20/enwiki_20180420_100d.pkl.bz2"
    },
    "googlenews2vec": {
        "file_name": "googlenews2vec.bin",
        "gdrive_link": "https://drive.google.com/u/1/uc?id=0B7XkCwpI5KDYNlNUTTlSS21pQmM&export=download"
    }
}

MODEL = "googlenews2vec"
MODEL_DIRECTORY = "model"

FULL_MODEL_PATH = (Path(__file__).parent.parent.resolve()
                   / MODEL_DIRECTORY
                   / MODEL_DICT[MODEL]["file_name"])

