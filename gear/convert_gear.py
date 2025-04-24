import os
import pathlib
import json

ignore_files = ["final_dict.json", "armor_sp.json", "calibers.json"]
simplify_keys = ["price", "internal", "psychosis", "burst", "damage", "fullauto", "rof", "skill", "weapontype"]
weap_data = ["burst", "damage", "fullauto", "rof", "skill", "weapontype", "ammo"]

base_delete = ["_id", "img", "permission", "flags", "effects"]
data_delete = ["rarity", "legality", "backend", "modlist", "temp","hackable"]

fashion_item_list = ["Bottoms", "Top", "Jacket", "Footwear", "Jewelry", "Mirrorshades", "Glasses", "Contact Lenses", "Hats"]

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

        output_data = []

        for item in data:
            add_to_output = True
            for key in base_delete:
                test_del(item, key)

            if "data" in item:
                itemdata: dict = item["data"]
                for key in simplify_keys:
                    simplify_value(itemdata, key)

                if fname == "weapons":
                    for key in weap_data:
                        move_weapon_data(item, itemdata, key)
                    replace_skill(item)

                for key in data_delete:
                    test_del(itemdata, key)

                for key in itemdata.keys():
                    item[key] = itemdata[key]
                del item["data"]

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
            output_data.append(item)

        if pathlib.Path.exists(new_path):
            os.remove(new_path)
        with open(new_path, "x") as new_file:
            new_file.write(json.dumps(output_data, indent=4, sort_keys=True))

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

        if fname == "fashion":
            handle_fashion_file(data, final_dict)

        elif fname == "ammunition_types":
            handle_ammo_file(data, final_dict)

        elif not ("type" in first_entry_data):
            raise Exception("object does not have type: " + fname)

        elif first_entry_data["type"] == "cyberware":
            if "cyberware" in final_dict:
                for entry in data:
                    final_dict["cyberware"].append(entry)
            else:
                final_dict["cyberware"] = data

        elif fname.find("program") != -1:
            if "programs" in final_dict:
                for entry in data:
                    final_dict["programs"].append(entry)
            else:
                final_dict["programs"] = data

        else:
            final_dict[fname] = data

    if pathlib.Path.exists(new_path):
        os.remove(new_path)
    with open(new_path, "x") as new_file:
        new_file.write(json.dumps(final_dict, indent=4, sort_keys=True))

def handle_fashion_file(fashion_file_list, output_dict):
    output_data = []
    for category in fashion_file_list:
        description = category["description"]
        cat_name = category["name"]
        for index, single_item in enumerate(fashion_item_list):
            price = category["item_prices"][index]
            single_fashion_item = {
                "name": single_item + " " + cat_name,
                "description": description,
                "price": price,
                "type": "fashion",
                "file": "fashion"
            }
            output_data.append(single_fashion_item)

    output_dict["fashion"] = output_data

def handle_ammo_file(ammo_file_list, output_dict):
    ammo_types = {}

    for ammo_type in ammo_file_list:
        selector = ammo_type["selector"]
        del ammo_type["selector"]
        ammo_types[selector] = ammo_type


    calibers_dict: dict = {}
    with open("calibers.json", "r") as file:
        calibers_dict = json.loads(file.read())

    output_data = []

    for caliber_key, caliber in calibers_dict.items():
        variants_list = caliber["variants"] if "variants" in caliber else [""]
        for variant in variants_list: 
            for ammo_type_key in caliber["allowed_types"]:

                ammo_type_data = {}
                for key, val in ammo_types[ammo_type_key].items():
                    ammo_type_data[key] = val

                full_ammo_name = variant + " " + caliber["name"] + " " + ammo_type_data["name"]
                full_caliber_name = variant.lower().replace(" ", "_").replace(".", "") + ("_" if variant != "" else "") + caliber_key

                ammo_type_data["name"] = full_ammo_name
                ammo_type_data["caliber"] = full_caliber_name
                ammo_type_data["only_one"] = caliber["only_one"]
                
                output_data.append(ammo_type_data)


    output_dict["ammunition"] = output_data

def main():
    convert_db()
    clean_data()
    combine_final_data()
    

if __name__ == "__main__":
    main()