import os
import pathlib
import json

def test_del(dictionary: dict, val:str):
    if val in dictionary:
        del dictionary[val]

# replaces last occurrence
def rreplace(s, old, new):
    return (s[::-1].replace(old[::-1],new[::-1], 1))[::-1]

def simplify_value(dictionary: dict, key:str):
    if not key in dictionary:
        return

    entry = dictionary[key]
    if not isinstance(entry, dict) or not "value" in entry:
        return

    val = entry["value"]
    dictionary[key] = val

if __name__ == "__main__":
    for file_name in os.listdir():
        print(file_name)

        spliced = file_name.split(".")

        if spliced[1] != "db":
            continue

        new_name = spliced[0] + ".json"
        new_path = pathlib.Path(new_name)

        with open(file_name, "r") as file:
            text: str = file.read()
            text = "[" + text.replace("\n", ",\n") + "]"
            text = rreplace(text, ",\n", "\n")
            if pathlib.Path.exists(new_path):
                os.remove(new_path)
            with open(new_path, "x") as new_file:
                new_file.write(text)

    for file_name in os.listdir():

        spliced = file_name.split(".")
        fname = spliced[0] 
        ftype = spliced[1] 

        if ftype != "json" or fname.find("_clean") != -1:
            continue

        print(file_name)

        new_path = pathlib.Path(file_name)
        data: list[dict] = None


        with open(file_name, "r") as file:
            data: list[dict] = json.loads(file.read())

        for item in data:
            test_del(item,"_id")
            test_del(item,"img")
            test_del(item,"permission")
            if "data" in item:
                itemdata = item["data"]
                simplify_keys = ["price", "legality", "rarity", "hackable", "internal", "psychosis", "burst", "damage", "fullauto", "rof", "skill", "weapontype"]
                for key in simplify_keys:
                    simplify_value(itemdata, key)
                test_del(itemdata,"backend")
                test_del(itemdata,"modlist")
                test_del(itemdata,"temp")
            item["file"] = fname

        print(data)
        if pathlib.Path.exists(new_path):
            os.remove(new_path)
        with open(new_path, "x") as new_file:
            new_file.write(json.dumps(data, indent=4, sort_keys=True))