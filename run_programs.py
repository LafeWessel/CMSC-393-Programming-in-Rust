import sys
import os
import json
import argparse
import glob


def get_project_list(projects=None):
    """Return list of all Rust project folders in current directory matching names in `projects`, or all if `projects` == None, not including those with '.'"""
    # get the names of all the folders in this directory, minus those with a '.'
    project_dirs = [f for f in os.listdir() if '.' not in f]

    # determine which project_dirs actually contain a Rust program
    project_dirs = [p for p in project_dirs if len(glob.glob(os.path.join(p, "**", "*.rs"))) > 0]

    # if projects is list, is several projects
    if type(projects) is list:
        return list(set([p for d in project_dirs for p in projects if p.lower() == d.lower()]))

    # if projects is not a list, return all projects
    return project_dirs


# TODO implement functionality for parameters.json
def generate_dockerfile(project_folder: str, no_overwrite=True):
    """Generate Dockerfile in `project_folder`, `no_overwrite` for determining whether to overwrite if the Dockerfile already exists"""
    dockerfile_path = os.path.join(project_folder, "Dockerfile")

    # return if no_overwrite == True and the Dockerfile exists
    if no_overwrite and os.path.isfile(dockerfile_path):
        print(f"Will not overwrite file {dockerfile_path}")
        return

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

    new_file = open(dockerfile_path, "w")
    new_file.write(temp)
    new_file.close()


def run_dockerfile(project_name: str):
    """Run Dockerfile from `project_name` directory"""
    print(f"Running Dockerfile for project {project_name}")

    # move into project_name folder
    starting_dir = os.getcwd()
    os.chdir(project_name)

    # run: docker run --rm --interactive --name proj_name proj_name
    cmd = f"docker run --rm --interactive --name {project_name.lower()} {project_name.lower()}"
    os.system(cmd)

    # move back to starting_dir
    os.chdir(starting_dir)


def clean_dockerfile(project_name: str):
    """Clean (Remove) Dockerfile from `project_name` directory"""
    dockerfile_path = os.path.join(project_name, "Dockerfile")

    if os.path.isfile(dockerfile_path):
        print(f"Removing {dockerfile_path}")
        os.remove(dockerfile_path)
    else:
        print(f"{dockerfile_path} unable to be removed as it does not exist.")


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


def run_script(args):
    """Run script based on command line arguments provided"""
    if sum([args.list, args.generate, args.run, args.clean_dockerfiles, args.build]) > 1:
        print("Too many mutually exclusive arguments provided, exiting")
        exit(1)

    if args.list:
        print("ONLY Listing projects, ignoring --project parameters")
        print(get_project_list())
        exit(1)

    if args.generate:
        print("ONLY Generating Dockerfiles")
        for p in get_project_list(args.project):
            generate_dockerfile(project_folder=p, no_overwrite=args.no_overwrite)
        exit(1)

    if args.run:
        print("ONLY Running Dockerfiles")
        for p in get_project_list(args.project):
            run_dockerfile(project_name=p)
        exit(1)

    if args.clean_dockerfiles:
        print("ONLY Cleaning Dockerfiles")
        for p in get_project_list(args.project):
            clean_dockerfile(project_name=p)
        exit(1)

    if args.build:
        print("ONLY Building Docker Containers")
        for p in get_project_list(args.project):
            build_container(project_name=p)
        exit(1)

    # run script normally
    for p in get_project_list(args.project):
        generate_dockerfile(project_folder=p, no_overwrite=args.no_overwrite)
        build_container(project_name=p)
        run_dockerfile(project_name=p)


def generate_args():
    """Generate ArgumentParser and return parsed arguments"""
    parser = argparse.ArgumentParser(description="Generate Dockerfiles for Rust programs and run them.")
    parser.add_argument("-p", "--project",
                        help="Run specific programs by name, if none provided, runs all programs",
                        action="append",
                        dest="project")
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


if __name__ == "__main__":
    args = generate_args()
    run_script(args=args)
