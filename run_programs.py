import sys
import os
import json
import argparse

# TODO add docstrings with """...""" to functions

# TODO add more logic to determine if a directory contains a rust program
# TODO add argument parsing for projects argument
def get_project_list():
    # get the names of all the folders in this directory
    proj = [f for f in os.listdir() if '.' not in f]
    return proj


def generate_dockerfile(project_folder: str, no_overwrite=True):
    print(f"Generating Dockerfile for project {project_folder}")

    # read in template_dockerfile
    temp = open("template_dockerfile.txt", "r").read()

    dockerfile_path = os.path.join(project_folder, "Dockerfile")
    parameters_path = os.path.join(project_folder, "parameters.json")

    # replace the $$ with project_name
    temp = temp.replace("$$", project_folder)

    # determine if parameters.json exists
    if os.path.isfile(parameters_path):
        print(f"Loading data from {parameters_path}")
        data = json.load(open(parameters_path, "r"))
    # end if

    # create new Dockerfile if no_overwrite == False, or if the Dockerfile does not exist
    if not no_overwrite or not os.path.isfile(dockerfile_path):
        new_file = open(dockerfile_path, "w")
        new_file.write(temp)
        new_file.close()
    # end if

    if no_overwrite and os.path.isfile(dockerfile_path):
        print(f"Did not overwrite file {dockerfile_path}")
    # end if
# end def


def run_dockerfile(project_name: str):
    print(f"Running Dockerfile for project {project_name}")

    # move into project_name folder
    starting_dir = os.getcwd()
    os.chdir(project_name)

    # run: docker build -t project_name .
    cmd = f"docker build -t {project_name.lower()} ."
    assert os.system(cmd) == 1

    # run: docker run --rm --interactive --name proj_name proj_name
    cmd = f"docker run --rm --interactive --name {project_name.lower()} {project_name.lower()}"
    assert os.system(cmd) == 1

    # move back to starting_dir
    os.chdir(starting_dir)
# end def

def clean_dockerfile(project_name: str):
    return

if __name__ == "__main__":

    parser = argparse.ArgumentParser(description="Generate Dockerfiles for Rust programs and run them.")
    parser.add_argument("-p", "--program",
                        help="Run specific programs by name, if none provided, runs all programs",
                        action="append",
                        dest="program")
    parser.add_argument("-l", "--list",
                        help="List all local projects and exit, defaults to False, mutually exclusive with -g,-l,-c",
                        action="store_true",
                        default=False,
                        dest="list")
    parser.add_argument("-g", "--generate",
                        help="Generate Dockerfiles and exit, defaults to False, mutually exclusive with -l,-r,-c",
                        action="store_true",
                        default=False,
                        dest="generate")
    parser.add_argument("-r", "--run",
                        help="Run projects with existing Dockerfiles, defaults to False, mutually exclusive with -l,-g,-c",
                        action="store_true",
                        default=False,
                        dest="run")
    parser.add_argument("-n", "--no-overwrite",
                        help="Do not overwrite already generated Dockerfiles, defaults to False",
                        action="store_true",
                        default=False,
                        dest="no_overwrite")
    parser.add_argument("-c","--clean-dockerfiles",
                        help="Clean Dockerfiles from project directories, defaults to False, mutually exclusive with -l,-g,-r",
                        action="store_true",
                        default=False,
                        dest="clean_dockerfiles")
    args = parser.parse_args()

    if args.list:
        print("ONLY Listing projects")
        print(get_project_list())
        exit(1)
    # end if

    if args.generate:
        print("ONLY Generating Dockerfiles")
        for p in get_project_list():
            generate_dockerfile(project_folder=p, no_overwrite=args.no_overwrite)
        # end for
        exit(1)
    # end if

    if args.run:
        print("ONLY Running Dockerfiles")
        for p in get_project_list():
            run_dockerfile(project_name=p)
        # end for
        exit(1)
    # end if

    if args.clean_dockerfiles:
        print("ONLY Cleaning Dockerfiles")
        for p in get_project_list():
            clean_dockerfile(project_name = p)
        # end for
        exit(1)
    # end if

    # run script normally
    for p in get_project_list():
        generate_dockerfile(project_folder=p, no_overwrite=args.no_overwrite)
        run_dockerfile()



# end def
