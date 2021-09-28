import sys
import os
import json
import argparse

# TODO add docstrings with """...""" to functions


def run_dockerfile(project_name: str):
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


# TODO add more logic to determine if a directory contains a rust program
# TODO add argument parsing for projects argument
def get_project_list():
    # get the names of all the folders in this directory
    proj = [f for f in os.listdir() if '.' not in f]
    return proj


def generate_dockerfile(project_folder: str, no_overwrite = True):

    print(f"Generating Dockerfile for project {project_folder}")

    # read in template_dockerfile
    temp = open("template_dockerfile.txt", "r").read()

    dockerfile_path = os.path.join(project_folder, "Dockerfile")
    parameters_path = os.path.join(project_folder, "parameters.json")

    # replace the $$ with project_name
    temp = temp.replace("$$", project_folder)

    # determine if parameters.json exists
    if os.path.isfile(parameters_path):
        print(f"Loading data in {parameters_path}")
        data = json.load(open("parameter.json", "r"))

    # create new Dockerfile if overwrite == True, or if the dockerfile does not exist
    if not no_overwrite or not os.path.isfile(dockerfile_path):
        new_file = open(dockerfile_path, "w")
        new_file.write(temp)
        new_file.close()

# end def


if __name__ == "__main__":

    parser = argparse.ArgumentParser(description="Generate Dockerfiles for Rust programs and run them.")
    parser.add_argument("-p", "--program",
                        help="Run specific programs by name, if none provided, runs all programs",
                        action="append")
    parser.add_argument("-l", "--list",
                        help="List all local projects and exit",
                        action="store_true",
                        default=False)
    parser.add_argument("-g", "--generate",
                        help="Generate Dockerfiles but do not run them",
                        action="store_true",
                        default=False)
    parser.add_argument("-n", "--no-overwrite",
                        help="Do not overwrite already generated Dockerfiles, defaults to False",
                        action="store_true",
                        default=False)
    parser.add_argument("-r", "--run",
                        help="Run projects without generating new Dockerfiles",
                        action="store_true",
                        default=False)
    args = parser.parse_args()

    if args.list:
        print("Listing projects only")
        print(get_project_list())
        exit(1)
    # end if

    if args.generate:
        print("Generating Dockerfiles only")
        for p in get_project_list():
            generate_dockerfile(project_folder=p, no_overwrite=args.no_overwrite)
        # end for
        exit(1)
    # end if

    if args.run:
        print("Running Dockerfiles only")
        for p in get_project_list():
            run_dockerfile(project_name=p)

    # run script normally
    for p in get_project_list():
        generate_dockerfile(project_folder=p, no_overwrite=args.no_overwrite)



# end def
