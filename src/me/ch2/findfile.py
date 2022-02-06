import os

from fire import Fire


def findfile(target_dir: str, keyword: str) -> None:
    """recursively find files in target_dir with keyword

    Args:
        target_dir ([type]): directory to look for files 
        keyword ([type]): the keyword to find
    """
    for dirname, dirs, files in os.walk(target_dir):
        for file in files:
            if keyword in file:
                fullpath = os.path.join(dirname, file)
                print(fullpath)


if __name__ == '__main__':
    Fire(findfile)