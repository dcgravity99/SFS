#!/usr/bin/env python3

import subprocess
import os
import sys


MODES = [
    ("story", "01_story"),
    ("director", "02_director"),
    ("camera", "03_camera"),
    ("producer", "04_production"),
    ("editor", "05_editing")
]


if len(sys.argv) < 2:
    print("Usage: film \"movie idea\"")
    exit()


idea = sys.argv[1]

project = "SIRA_FILM_PROJECT"

os.makedirs(project, exist_ok=True)


for mode, folder in MODES:

    print("\nGenerating:", mode)

    output = subprocess.check_output(
        [
            "python3",
            "sira/sira.py",
            mode,
            idea
        ],
        text=True
    )

    path = os.path.join(project, folder)

    os.makedirs(path, exist_ok=True)

    with open(
        path + "/" + mode + ".md",
        "w"
    ) as file:

        file.write(output)


print("\nSIRA FILM PROJECT COMPLETE")
print(project)
