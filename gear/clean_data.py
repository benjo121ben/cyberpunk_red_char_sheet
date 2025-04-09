import os
import pathlib
import json

def test_del(dictionary: dict, val:str):
    if val in dictionary:
        del dictionary[val]

if __name__ == "__main__":
    for file_name in os.listdir():

        spliced = file_name.split(".")

        if spliced[1] != "json" or ("_clean" in spliced[1]):
            continue

        print(file_name)

        new_name = spliced[0] + "_clean" + ".json"
        new_path = pathlib.Path(new_name)

        with open(file_name, "r") as file:
            data: list[dict] = json.loads(file.read())
            for item in data:
                test_del(item,"_id")
                test_del(item,"img")
                test_del(item,"permission")
                if "data" in item:
                    itemdata = item["data"]
                    test_del(itemdata,"backend")
                    test_del(itemdata,"temp")
            print(data)
            if pathlib.Path.exists(new_path):
                os.remove(new_path)
            with open(new_path, "x") as new_file:
                new_file.write(json.dumps(data, indent=4, sort_keys=True))