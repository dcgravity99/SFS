import os


def create_project(name):

    folders = [

        "01_story",
        "02_screenplay",
        "03_director",
        "04_camera",
        "05_production",
        "06_editing",
        "07_vfx",
        "08_sound"

    ]


    root = os.path.join(
        os.getcwd(),
        "PROJECTS",
        name
    )


    os.makedirs(root, exist_ok=True)


    for folder in folders:

        os.makedirs(
            os.path.join(root, folder),
            exist_ok=True
        )


    print("🎬 SIRA Project Created")
    print(root)
