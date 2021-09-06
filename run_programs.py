import sys
import os

# get the names of all the folders in this directory
proj = [f for f in os.listdir() if '.' not in f]

print(f"Projects: {proj}")

for p in proj:
    print(f"Running {p}")
    # read in template_dockerfile
    temp = open("template_dockerfile.txt", "r").read()

    # replace the $$ with proj
    temp = temp.replace("$$", p)

    # enter project folder
    os.chdir(p)

    # create new Dockerfile
    new_file = open("Dockerfile","w")
    new_file.write(temp)
    new_file.close()

    # run: docker build -t project_name .
    cmd = f"docker build -t {p.lower()} ."
    os.system(cmd)

    # run: docker run --rm --interactive --name proj_name proj_name
    cmd = f"docker run --rm --interactive --name {p.lower()} {p.lower()}"
    os.system(cmd)

    # move back a directory
    os.chdir("..")
# end for

