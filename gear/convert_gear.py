import os
import pathlib
import json

ignore_files = ["final_dict.json", "armor_sp.json"]

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

def move_weapon_data(base_dict: dict, data_dict: dict, key: str):
    if key not in data_dict:
        return
    
    entry = data_dict[key]
    if "weapon_data" not in base_dict:
        base_dict["weapon_data"] = {}
    weapon_data_dict = base_dict["weapon_data"]
    weapon_data_dict[key] = entry
    del data_dict[key]

def replace_skill(base_dict: dict):
    if "weapon_data" not in base_dict:
        return
    
    weap_data = base_dict["weapon_data"]

    if "skill" not in weap_data:
        return
    
    entry: str = weap_data["skill"]
    weap_data["skill"] = entry.replace("meleeweapon", "melee_weapon").replace("shoulderarms", "shoulder_arms").replace("heavyweapons", "heavy_weapons")

def convert_db():
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

def clean_data():
    armor_sp_data = None
    with open("armor_sp.json") as armor_file:
        armor_sp_data = json.loads(armor_file.read())

    for file_name in os.listdir():

        spliced = file_name.split(".")
        fname = spliced[0] 
        ftype = spliced[1] 

        if ftype != "json" or file_name in ignore_files:
            continue

        print(file_name)

        new_path = pathlib.Path(file_name)
        data: list[dict] = None


        with open(file_name, "r") as file:
            data: list[dict] = json.loads(file.read())

        simplify_keys = ["price", "legality", "rarity", "hackable", "internal", "psychosis", "burst", "damage", "fullauto", "rof", "skill", "weapontype"]
        weap_data = ["burst", "damage", "fullauto", "rof", "skill", "weapontype", "ammo"]

        for item in data:
            test_del(item,"_id")
            test_del(item,"img")
            test_del(item,"permission")
            test_del(item,"flags")
            test_del(item,"effects")
            if "data" in item:
                itemdata = item["data"]
                for key in simplify_keys:
                    simplify_value(itemdata, key)

                if fname == "weapons":
                    for key in weap_data:
                        move_weapon_data(item, itemdata, key)
                    replace_skill(item)

                test_del(itemdata,"backend")
                test_del(itemdata,"modlist")
                test_del(itemdata,"temp")

            if fname == "armor":
                name = item["name"]
                armor_sp_entry = armor_sp_data[name]
                if "is_shield" in armor_sp_entry:
                    del armor_sp_entry["is_shield"]
                    item["type"] = "shield"
                else:
                    item["type"] = "armor"
                item["armor_data"] = armor_sp_entry

            item["file"] = fname

        if pathlib.Path.exists(new_path):
            os.remove(new_path)
        with open(new_path, "x") as new_file:
            new_file.write(json.dumps(data, indent=4, sort_keys=True))

def combine_final_data():
    final_dict = {}
    new_path = pathlib.Path("final_dict.json")

    print("--------------------------")
    for file_name in os.listdir():
        spliced = file_name.split(".")
        fname = spliced[0] 
        ftype = spliced[1] 

        if ftype != "json" or file_name in ignore_files:
            continue

        print(file_name)

        data: list[dict] = None

        with open(file_name, "r") as file:
            data: list[dict] = json.loads(file.read())

        first_entry_data = data[0]
        if not ("type" in first_entry_data) or first_entry_data["type"] != "cyberware":
            final_dict[fname] = data
        elif "cyberware" in final_dict:
            for entry in data:
                final_dict["cyberware"].append(data)
        else:
            final_dict["cyberware"] = data

    if pathlib.Path.exists(new_path):
        os.remove(new_path)
    with open(new_path, "x") as new_file:
        new_file.write(json.dumps(final_dict, indent=4, sort_keys=True))

def main():
    convert_db()
    clean_data()
    combine_final_data()
    

if __name__ == "__main__":
    main()