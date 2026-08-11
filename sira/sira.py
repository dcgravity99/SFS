#!/usr/bin/env python3

import sys
import os
import requests
import json
import subprocess

BASE_DIR = os.path.expanduser("~/Siragugal")

PROMPT_DIR = os.path.join(
    BASE_DIR,
    "sira",
    "prompts"
)

LLM_URL = "http://127.0.0.1:8080/v1/chat/completions"


MODES = {

    "story": "story_architect.txt",

    "screenwriter": "screenwriter_mode.txt",

    "director": "director_mode.txt",

    "camera": "cinematographer_mode.txt",

    "producer": "producer_mode.txt",

    "editor": "editor_mode.txt"

}


def load_prompt(mode):

    if mode not in MODES:
        print("\nUnknown SIRA mode:", mode)

        print("\nAvailable modes:")

        for m in MODES:
            print("-", m)

        sys.exit(1)


    file_path = os.path.join(
        PROMPT_DIR,
        MODES[mode]
    )


    if not os.path.exists(file_path):

        print("Prompt file missing:")
        print(file_path)

        sys.exit(1)


    with open(file_path, "r") as f:

        return f.read()



def ask_sira(mode, request):

    system_prompt = load_prompt(mode)


    payload = {

        "messages": [

            {
                "role": "system",
                "content": system_prompt
            },

            {
                "role": "user",
                "content": request
            }

        ],

        "temperature": 0.7,

        "max_tokens": 2000

    }


    response = requests.post(
        LLM_URL,
        json=payload
    )


    if response.status_code != 200:

        print(response.text)

        sys.exit(1)


    data = response.json()


    print(
        data["choices"][0]["message"]["content"]
    )



def run_film_pipeline(request):

    print("\n🎬 SIRA FILM PIPELINE START\n")


    modules = [

        ("story", "story"),

        ("director", "director"),

        ("camera", "camera"),

        ("producer", "producer"),

        ("editor", "editor")

    ]


    output = "SIRA_FILM_PROJECT"

    os.makedirs(output, exist_ok=True)


    for folder, mode in modules:

        print("Generating:", folder)


        result_file = os.path.join(
            output,
            folder
        )


        with open(result_file, "w") as f:

            old_stdout = sys.stdout

            sys.stdout = f

            ask_sira(
                mode,
                request
            )

            sys.stdout = old_stdout



    print("\n🎬 SIRA FILM PROJECT COMPLETE")

    print(output)



def main():

    if len(sys.argv) < 3:

        print("""
Usage:

sira <mode> "request"


Modes:

story
screenwriter
director
camera
producer
editor
film
""")

        sys.exit(1)



    mode = sys.argv[1]

    request = sys.argv[2]



    if mode == "film":

        run_film_pipeline(request)

    else:

        ask_sira(
            mode,
            request
        )



if __name__ == "__main__":

    main()
