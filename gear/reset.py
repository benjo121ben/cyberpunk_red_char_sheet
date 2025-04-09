import os
import pathlib
import json

def test_del(dictionary: dict, val:str):
    if val in dictionary:
        del dictionary[val]

if __name__ == "__main__":
    for file_name in os.listdir():

        spliced = file_name.split(".")

        if spliced[1] != "json":
            continue

        os.remove(pathlib.Path(file_name))