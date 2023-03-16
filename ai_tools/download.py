import gzip
import shutil
import urllib.request

import gdown
from tqdm import tqdm

from ai_tools.settings import MODEL_DICT, FULL_MODEL_PATH


class DownloadProgressBar(tqdm):
    def update_to(self, b=1, bsize=1, tsize=None):
        if tsize is not None:
            self.total = tsize
        self.update(b * bsize - self.n)


def download_model(model_name) -> None:
    """
    Checks if the model file exists, and downloads it if not.
    """
    print(FULL_MODEL_PATH)
    assert model_name in MODEL_DICT, "Not a valid model name."

    if FULL_MODEL_PATH.is_file():
        return

    if "http_link" in MODEL_DICT[model_name]:
        with DownloadProgressBar(
                unit='B',
                unit_scale=True,
                miniters=1,
                desc=MODEL_DICT[model_name].split('/')[-1]
        ) as t:
            urllib.request.urlretrieve(
                MODEL_DICT[model_name]["http_link"],
                filename=FULL_MODEL_PATH,
                reporthook=t.update_to
            )
    if "gdrive_link" in MODEL_DICT[model_name]:
        gdown.download(
            MODEL_DICT[model_name]["gdrive_link"],
            FULL_MODEL_PATH.as_posix()
        )

        with gzip.open(FULL_MODEL_PATH, 'rb') as f_in:
            with open(FULL_MODEL_PATH.with_suffix(".bin"), 'wb') as f_out:
                shutil.copyfileobj(f_in, f_out)
