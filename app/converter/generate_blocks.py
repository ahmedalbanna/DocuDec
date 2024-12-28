import os
import platform
import subprocess
from concurrent.futures import ThreadPoolExecutor

from .extract_plantuml import extract_plantuml_code
from .prepare_blocks import prepare_blocks


# from .call_gemini_pro import code_generator  # Uncomment if using Gemini

def getDotDir():
    where_commad = subprocess.run(["where", "dot"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    stdout: str = where_commad.stdout
    dot = stdout.splitlines()[0]
    return dot


def check_plantuml_syntax(plantuml_code):
    """Checks the syntax of PlantUML code. Returns True if correct, False otherwise."""
    try:
        syntax = subprocess.run(
            ["plantuml", "-syntax"],
            input=plantuml_code.encode(),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=True,
        )
        return syntax.stderr  # Syntax is correct
    except subprocess.CalledProcessError:
        return False  # Syntax error


def generate_image(block, i, blocks_folder, images_folder):
    """Generates an image for a single PlantUML block."""
    plantuml_file = os.path.join(blocks_folder, f"block{i}.plantuml")

    command = [
        "plantuml",
        "-tpng",
        plantuml_file,
        "-o",
        images_folder,
    ]
    if platform.system() == "Windows":
        command.insert(2, "-graphvizdot")
        command.insert(3, getDotDir())

    try:
        subprocess.run(command, check=True)
        print(f"Generated image for block {i}")
    except subprocess.CalledProcessError as e:
        print(f"Error generating image for block {i}: {e}")


def make_blocks(filename, blocks_folder, images_folder):
    """Extracts PlantUML blocks, generates images, and updates the file."""
    plantuml_code_blocks = extract_plantuml_code(filename)

    # Fix PlantUML syntax errors (if using Gemini)
    # Check syntax before generating images
    for i, block in enumerate(plantuml_code_blocks, start=1):
        if check_plantuml_syntax(block) != b'':
            print(f"Syntax error in PlantUML block {i}!")
            # # Send to Gemini to fix
            fixed_block = True  # code_generator(block)  # Assuming code_generator is implemented
            if fixed_block:
                plantuml_code_blocks[i - 1] = """@startuml
                                                "NOT Completed"
                                                @enduml"""  # fixed_block
                print(f"PlantUML block {i} fixed by Gemini.")

            else:
                print(f"Failed to fix PlantUML block {i}.")
                # You might want to handle this error more gracefully (e.g., skip the block, prompt the user)
                return

    prepare_blocks(filename, plantuml_code_blocks, blocks_folder, images_folder)

    # Generate images concurrently
    with ThreadPoolExecutor() as executor:
        executor.map(
            generate_image,
            plantuml_code_blocks,
            range(1, len(plantuml_code_blocks) + 1),
            [blocks_folder] * len(plantuml_code_blocks),
            [images_folder] * len(plantuml_code_blocks),
        )
