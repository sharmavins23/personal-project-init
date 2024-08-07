import os
import time


GRAY = "\u001b[38;5;248m"
CYAN = "\u001b[36m"
RED = "\u001b[31m"
GREEN = "\u001b[32m"
YELLOW = "\u001b[33m"
RESET = "\u001b[0m"


# Get the path name of where the shell the script was called from
path = os.getcwd()
print(f"Found path {CYAN}{path}{RESET}.")

# Also get the program path
program_path = os.path.dirname(os.path.realpath(__file__))
print(f"Command running from {GREEN}{program_path}{RESET}.")

# Strip after the last '\' to get the directory's name
directory = path.split("\\")[-1]
# Convert to camel case, splitting on '-' or '_'
directory = " ".join([word.capitalize() for word in directory.split("-")])
directory = " ".join([word.capitalize() for word in directory.split("_")])

# Create a README.md file in the directory
with open("README.md", "w") as f:
    f.write(f"# {directory}\n\n")

    # Pull the text from the README.md file in the /data/ directory
    with open(f"{program_path}/data/README.md", "r") as data:
        f.write(data.read())

# Create a LICENSE.md file in the directory
with open("LICENSE.md", "w") as f:
    f.write("MIT License\n\n")
    # Get the current year
    year = time.localtime().tm_year
    f.write(f"Copyright (c) {year} sharmavins23\n\n")

    # Pull the text from the LICENSE.md file in the /data/ directory
    with open(f"{program_path}/data/LICENSE.md", "r") as data:
        f.write(data.read())

# Create a .gitignore file in the directory based on the /data/ directory
with open(".gitignore", "w") as f:
    with open(f"{program_path}/data/.gitignore", "r") as data:
        f.write(data.read())

# Finally, run `git init` in the directory and upload the initial commit
os.system("git init")
os.system("git add *")
os.system("git commit -m 'Initial commit'")
