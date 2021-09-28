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
    dockerfile_path = os.path.join(project_folder, "Dockerfile")

    # return if no_overwrite == True and the Dockerfile exists
    if no_overwrite and os.path.isfile(dockerfile_path):
        print(f"Will not overwrite file {dockerfile_path}")
        return
    # end if

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

    new_file = open(dockerfile_path, "w")
    new_file.write(temp)
    new_file.close()
# end def


def run_dockerfile(project_name: str):
    """Run Dockerfile from `project_name` directory"""
    print(f"Running Dockerfile for project {project_name}")

    # move into project_name folder
    starting_dir = os.getcwd()
    os.chdir(project_name)

    # run: docker run --rm --interactive --name proj_name proj_name
    cmd = f"docker run --rm --interactive --name {project_name.lower()} {project_name.lower()}"
    assert os.system(cmd) == 1

    # move back to starting_dir
    os.chdir(starting_dir)
# end def


def clean_dockerfile(project_name: str):
    """Clean (Remove) Dockerfile from `project_name` directory"""
    dockerfile_path = os.path.join(project_name, "Dockerfile")

    if os.path.isfile(dockerfile_path):
        print(f"Removing {dockerfile_path}")
        os.remove(dockerfile_path)
    # end if
    else:
        print(f"{dockerfile_path} unable to be removed as it does not exist.")
# end def


def build_container(project_name: str):
    """Build Docker container for `project_name` project in that directory"""
    print(f"Building Docker container for {project_name}")

    # move into project_name folder
    starting_dir = os.getcwd()
    os.chdir(project_name)

    # run: docker build -t project_name .
    cmd = f"docker build -t {project_name.lower()} ."
    os.system(cmd)

    # jump back to starting_dir
    os.chdir(starting_dir)
# end def

def run_script(args):
    """Run script based on command line arguments provided"""
    if sum([args.list, args.generate, args.run, args.clean_dockerfiles, args.build]) > 1:
        print("Too many mutually exclusive arguments provided, exiting")
        exit(1)
    # end if

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
            clean_dockerfile(project_name=p)
        # end for
        exit(1)
    # end if

    if args.build:
        print("ONLY Building Docker Containers")
        for p in get_project_list():
            build_container(project_name=p)
        # end for
        exit(1)
    # end if

    # run script normally
    for p in get_project_list():
        generate_dockerfile(project_folder=p, no_overwrite=args.no_overwrite)
        run_dockerfile()
    # end for
# end def


def generate_args():
    """Generate ArgumentParser and return parsed arguments"""
    parser = argparse.ArgumentParser(description="Generate Dockerfiles for Rust programs and run them.")
    parser.add_argument("-p", "--program",
                        help="Run specific programs by name, if none provided, runs all programs",
                        action="append",
                        dest="program")
    parser.add_argument("-l", "--list",
                        help="List all local projects and exit, defaults to False, mutually exclusive with -g,-l,-c,-b",
                        action="store_true",
                        default=False,
                        dest="list")
    parser.add_argument("-g", "--generate",
                        help="Generate Dockerfiles and exit, defaults to False, mutually exclusive with -l,-r,-c,-b",
                        action="store_true",
                        default=False,
                        dest="generate")
    parser.add_argument("-r", "--run",
                        help="Run projects with existing Dockerfiles, defaults to False, mutually exclusive with -l,-g,-c,-b",
                        action="store_true",
                        default=False,
                        dest="run")
    parser.add_argument("-n", "--no-overwrite",
                        help="Do not overwrite already generated Dockerfiles, defaults to False, used with -g and default behavior",
                        action="store_true",
                        default=False,
                        dest="no_overwrite")
    parser.add_argument("-c","--clean-dockerfiles",
                        help="Clean Dockerfiles from project directories, defaults to False, mutually exclusive with -l,-g,-r,-b",
                        action="store_true",
                        default=False,
                        dest="clean_dockerfiles")
    parser.add_argument("-b", "--build",
                        help="Build Docker containers, defaults to False, mutually exclusive with -l,-g,-r,-b",
                        action="store_true",
                        default=False,
                        dest="build")
    return parser.parse_args()
# end def


if __name__ == "__main__":
    args = generate_args()
    run_script(args=args)
# end def
