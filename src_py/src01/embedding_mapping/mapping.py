import umap
import numpy as np

def csv_to_ndarray(path: str) -> np.ndarray:
    arr = np.loadtxt(path)
    return arr
