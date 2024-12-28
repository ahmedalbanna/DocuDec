import os


def makeMissingFolders(folders):
    """Creates missing folders if they don't exist.

    Args:
        folders (list): A list of folder paths to create.
    """
    for folder in folders:
        os.makedirs(folder, exist_ok=True)
