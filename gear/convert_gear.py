import os
import pathlib

# replaces last occurrence
def rreplace(s, old, new):
    return (s[::-1].replace(old[::-1],new[::-1], 1))[::-1]

if __name__ == "__main__":
    for file_name in os.listdir():
        print(file_name)

        spliced = file_name.split(".")

        if spliced[1] != "db":
            continue

        new_name = spliced[0] + ".json"
        new_path = pathlib.Path(new_name);

        with open(file_name, "r") as file:
            text: str = file.read()
            text = "[" + text.replace("\n", ",\n") + "]"
            text = rreplace(text, ",\n", "\n")
            if pathlib.Path.exists(new_path):
                os.remove(new_path)
            with open(new_path, "x") as new_file:
                new_file.write(text)