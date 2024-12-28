import os
import time


def deleteLeftOvers(folders: list):
    time.sleep(5)
    for folder in folders:
        for root, dirs, files in os.walk(folder, topdown=False):
            for file in files:
                os.remove(os.path.join(root, file))
            for _dir in dirs:
                os.rmdir(os.path.join(root, _dir))
