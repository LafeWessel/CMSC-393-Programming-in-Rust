# Programming in Rust Individual Studies

This project contains all the code that I have written for my Individual Studies course in which I am teaching myself the Rust programming language.

## Run Programs

To run the programs, you must have Docker and Python 3.* installed on your system. Once you have these installed, run the `run_programs.py` file, which will iterate through the projects, create a Dockerfile for each of them, build the container, run it, then close and remove the containers afterwards. You can run the containers individually by using the appropriate `docker build` and `docker run` commands: these commands are listed in the `run_programs.py` script.

## Notes

Running the `run_programs.py` script will run all the local programs, and there is not yet a way to modify this behavior.