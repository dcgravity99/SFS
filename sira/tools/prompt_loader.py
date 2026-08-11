import os


PROMPT_DIR = os.path.join(
    os.path.dirname(os.path.dirname(__file__)),
    "prompts"
)


def load_prompt(mode):

    core_file = os.path.join(
        PROMPT_DIR,
        "sira_core.txt"
    )

    mode_file = os.path.join(
        PROMPT_DIR,
        f"{mode}.txt"
    )


    with open(core_file, "r") as f:
        core = f.read()


    with open(mode_file, "r") as f:
        mode_prompt = f.read()


    return core + "\n\n" + mode_prompt



if __name__ == "__main__":

    prompt = load_prompt(
        "story_architect"
    )

    print(prompt)
