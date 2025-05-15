import os
import pathlib
import json

simplify_keys = ["price", "internal", "psychosis", "burst", "damage", "fullauto", "rof", "skill", "weapontype"]
weap_data = ["burst", "damage", "fullauto", "rof", "skill", "weapontype", "ammo"]
base_data_keys = ["name", "price", "description", "file", "type"]

base_delete = ["_id", "img", "permission", "flags", "effects"]
data_delete = ["rarity", "legality", "backend", "modlist", "temp","hackable"]

fashion_item_list = ["Bottoms", "Top", "Jacket", "Footwear", "Jewelry", "Mirrorshades", "Glasses", "Contact Lenses", "Hats"]

def get_relative_path(*values):
    return pathlib.Path(os.path.join(os.path.dirname(__file__), *values))

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

def load_file(path): 
    if not pathlib.Path.exists(path):
        raise Exception("file " + path + " does not exist.")
    with open(path, "r") as file:
        return json.loads(file.read())
    

def save_file(output_data, new_path):
    if pathlib.Path.exists(new_path):
        os.remove(new_path)
    with open(new_path, "x") as new_file:
        new_file.write(json.dumps(output_data, indent=4, sort_keys=True))

def add_file_data():
    complex_item_folder_path = os.path.join(os.path.dirname(__file__), 'complex_items')
    for file_name in os.listdir(complex_item_folder_path):
        fname = file_name.split(".")[0] 

        print(file_name)
        new_path = pathlib.Path("cl_" + file_name)

        item_path = complex_item_folder_path = pathlib.Path(os.path.join(os.path.dirname(__file__), 'complex_items', file_name))
        data: list[dict] = load_file(item_path)
        output_data = []

        for item in data:
            item["file"] = fname
            output_data.append(item)

        save_file(output_data, new_path)
        
def combine_final_data():
    final_dict = {}
    new_path = pathlib.Path("final_dict.json")

    print("--------------------------")

    handle_complex_files(final_dict)
    handle_simple_files(final_dict)
    handle_fashion_file(final_dict)
    handle_ammo_file(final_dict)

    save_file(final_dict, new_path)

def handle_complex_files(final_dict):
    for filename in os.listdir():
        print(filename)

        if not filename.startswith("cl_"):
            continue

        fname = filename[3:].split('.')[0]
        print("test", fname)
        data: list[dict] = load_file(pathlib.Path(filename))
        first_entry_data = data[0]

        if not ("type" in first_entry_data):
            raise Exception("object does not have type: " + fname)

        elif first_entry_data["type"] == "cyberware":
            if "cyberware" in final_dict:
                for entry in data:
                    final_dict["cyberware"].append(entry)
            else:
                final_dict["cyberware"] = data
        else:
            final_dict[fname] = data

        os.remove(filename)

def handle_simple_files(final_dict):
    for filename in os.listdir(get_relative_path("simple_items")):
        data: list[dict] = load_file(get_relative_path("simple_items", filename))
        fname = filename.split(".")[0]
        if fname.find("program") != -1:
            if "programs" in final_dict:
                for entry in data:
                    final_dict["programs"].append(entry)
            else:
                final_dict["programs"] = data

        else:
            final_dict[fname] = data

def handle_fashion_file(output_dict):
    fashion_path = get_relative_path('generation_data', 'fashion.json')
    fashion_file_list = load_file(fashion_path)
    output_data = []

    for category in fashion_file_list:
        description = category["description"]
        cat_name = category["name"]
        for index, single_item in enumerate(fashion_item_list):
            price = category["item_prices"][index]
            single_fashion_item = {
                "name": single_item + " - " + cat_name,
                "description": description,
                "price": price,
                "type": "fashion",
                "file": "fashion"
            }
            output_data.append(single_fashion_item)

    output_dict["fashion"] = output_data

def handle_ammo_file(output_dict):
    ammunition_path = get_relative_path('generation_data', 'ammunition_types.json')
    caliber_path = get_relative_path('generation_data', 'calibers.json')

    ammo_file_list = load_file(ammunition_path)
    calibers_dict = load_file(caliber_path)

    ammo_types = {}
    for ammo_type in ammo_file_list:
        selector = ammo_type["selector"]
        del ammo_type["selector"]
        ammo_types[selector] = ammo_type

    output_data = []

    for caliber_key, caliber_data in calibers_dict.items():
        variants_list = caliber_data["variants"] if "variants" in caliber_data else [""]
        for variant in variants_list: 
            add_caliber_variant_to_output(output_data, ammo_types, caliber_key, caliber_data, variant)

    output_dict["ammunition"] = output_data



def add_caliber_variant_to_output(output_data, ammo_types, caliber_key, caliber_data, variant):
    # ammo types where the caliber is put after the ammo type and we remove "Ammunition" from the name: like "Gas Grenade" instead of "Grenade Gas Ammunition"
    reverse_names = ["grenade", "arrow", "shell"] 

    for ammo_type_key in caliber_data["allowed_types"]:
        # we make a clone of the ammo type data (name, price, descr, ecc.) in order to avoid changing the original data
        ammo_type_data = {}
        for key, val in ammo_types[ammo_type_key].items():
            ammo_type_data[key] = val

        full_ammo_name = variant + " " + caliber_data["name"] + " - " + ammo_type_data["name"]
        if caliber_key in reverse_names:
            full_ammo_name = ammo_type_data["name"] + " " + caliber_data["name"]
            full_ammo_name = full_ammo_name.replace("Ammunition ", "")
            
        full_ammo_name = full_ammo_name.strip()

        if caliber_key == "shell":
            full_ammo_name = full_ammo_name.replace("Basic ", "")
        
        full_caliber_name = variant.lower().replace(" ", "_").replace(".", "") + ("_" if variant != "" else "") + caliber_key

        ammo_type_data["name"] = full_ammo_name
        ammo_type_data["caliber"] = full_caliber_name
        ammo_type_data["only_one"] = caliber_data["only_one"]
        
        output_data.append(ammo_type_data)


def main():
    add_file_data()
    combine_final_data()
    

if __name__ == "__main__":
    main()