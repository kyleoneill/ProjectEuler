from os import listdir
from os import path
import os

def edit_file(file_name):
    file = open(file_name)
    file_contents = file.readlines()
    file.close()
    for i, line in enumerate(file_contents):
        if "name" in line:
            file_contents[i] = line.replace('rs_', '')
    write_file = open(file_name, "w")
    new_contents = "".join(file_contents)
    write_file.write(new_contents)
    write_file.close()

def main():
    folders = os.listdir()
    for folder in folders:
        cargo_lock = folder + "/Cargo.lock"
        cargo_toml = folder + "/Cargo.toml"
        if(path.exists(cargo_lock)):
            os.remove(cargo_lock)
        if(path.exists(cargo_toml)):
            edit_file(cargo_toml)
        if 'rs_' in folder:
            os.rename(folder, folder.replace('rs_', ''))
        

main()
