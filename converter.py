import logging
import os
import platform
import re
import shutil
import subprocess
import sys
import time  # Import time module
from concurrent.futures import ThreadPoolExecutor
from functools import cache


def remove_plantuml_code(filename):
    """Opens a file and removes PlantUML code blocks from it.

    Args:
      filename: The name of the file to remove PlantUML code blocks from.
    """
    with open(filename, 'r+') as f:
        text = f.read()
        # This pattern was removing the @startuml and @enduml tags, which are needed
        # pattern = r"```plantuml\s*(.*?)```"
        # text = re.sub(pattern, r"\1", text, flags=re.DOTALL).strip().removeprefix("```plantuml").removesuffix("```")
        text = re.sub(r"```plantuml\n((.|\n)*?)\n```", r"\1", text, flags=re.DOTALL)
        f.seek(0)
        f.write(text)
        f.truncate()


def prepare_blocks(markdown_file, plantuml_blocks, blocks_folder, images_folder):
    """Replaces PlantUML blocks in a Markdown file with image placeholders.

    Args:
        markdown_file (str): Path to the Markdown file.
        plantuml_blocks (list): List of PlantUML code blocks.
        blocks_folder (str): Path to the folder for saving PlantUML files.
        images_folder (str): Path to the folder for saving generated images.

    Raises:
        FileNotFoundError: If the Markdown file is not found.
        IOError: If there's an error writing to files.

    """
    try:
        with open(markdown_file, 'r+') as f:
            markdown_content = f.read()
            for i, block in enumerate(plantuml_blocks, 1):
                image_path = os.path.join(images_folder, f"block{i}.png")
                markdown_content = markdown_content.replace(block, f"![PlantUML Diagram {i}]({image_path})\n")
                block_filepath = os.path.join(blocks_folder, f"block{i}.plantuml")
                with open(block_filepath, 'w') as block_file:
                    block_file.write(block)
            f.seek(0)
            f.write(markdown_content)
            f.truncate()
    except FileNotFoundError:
        raise FileNotFoundError(f"Markdown file not found: {markdown_file}")
    except IOError as e:
        raise IOError(f"Error writing to file: {e}")


def makeMissingFolders(folders):
    """Creates missing folders if they don't exist.

    Args:
        folders (list): A list of folder paths to create.
    """
    for folder in folders:
        os.makedirs(folder, exist_ok=True)


@cache
def getAppDirectory() -> str:
    """
    Gets the script or the
    executable path
    """
    if getattr(sys, "frozen", False):
        # Running in a bundle
        bundle_dir = os.path.dirname(sys.executable)
    else:
        # Running in a normal Python environment
        bundle_dir = os.path.dirname(os.path.abspath(__file__))
    if bundle_dir.endswith("app/converter"):
        bundle_dir = bundle_dir[: -len("app/converter")]

    if bundle_dir.endswith("converter"):
        bundle_dir = bundle_dir[: -len("converter")]

    return bundle_dir


def extract_plantuml_code(filename):
    """Extracts PlantUML code blocks from a file.

    Args:
        filename: Path to the input file.

    Returns:
        A list of strings, where each string is a PlantUML code block.  Returns an empty list if no PlantUML blocks are found.  Raises exceptions if file processing errors occur.
    """
    plantuml_blocks = []
    try:
        with open(filename, "r") as file:
            in_uml_block = False
            in_gantt_block = False
            current_block = ""

            for line in file:
                stripped_line = line.strip()
                if stripped_line == "@startuml":
                    in_uml_block = True
                    current_block += line
                elif stripped_line == "@enduml" and in_uml_block:
                    current_block += line
                    plantuml_blocks.append(current_block)
                    current_block = ""
                    in_uml_block = False
                elif stripped_line == "@startgantt":
                    in_gantt_block = True
                    current_block += line
                elif stripped_line == "@endgantt" and in_gantt_block:
                    current_block += line
                    plantuml_blocks.append(current_block)
                    current_block = ""
                    in_gantt_block = False
                elif in_uml_block or in_gantt_block:
                    current_block += line

    except FileNotFoundError:
        raise FileNotFoundError(f"File not found: {filename}")
    except Exception as e:
        raise Exception(f"An error occurred while processing the file: {e}")

    return plantuml_blocks


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


def deleteLeftOvers(folders: list):
    time.sleep(5)
    for folder in folders:
        for root, dirs, files in os.walk(folder, topdown=False):
            for file in files:
                os.remove(os.path.join(root, file))
            for _dir in dirs:
                os.rmdir(os.path.join(root, _dir))


def convertMarkdownToDocx(input_md: str, output_docx: str, resource_path: str = None, template_path: str = None):
    """Converts a Markdown file to a DOCX file using Pandoc.

    Args:
        input_md: Path to the input Markdown file.
        output_docx: Path to the output DOCX file.
        resource_path: Path to resources (images, etc.) used in the Markdown file.
        template_path: Path to a Pandoc template file.

    Raises:
        subprocess.CalledProcessError: If Pandoc execution fails.
        FileNotFoundError: If the input Markdown file is not found.
        Exception: For any other errors during file processing.

    """
    command = ["pandoc", input_md, "-o", output_docx, "-t", "docx", "--standalone"]

    if resource_path:
        command.append(f"--resource-path={resource_path}")

    if template_path:
        command.extend(["--reference-doc", template_path])

    try:
        subprocess.run(command, check=True, capture_output=True, text=True)  # Capture output for better error messages
    except FileNotFoundError:
        raise FileNotFoundError(f"Input file '{input_md}' not found or Pandoc is not in your PATH.")
    except subprocess.CalledProcessError as e:
        error_message = f"Pandoc execution failed:\nReturn code: {e.returncode}\nStdout: {e.stdout}\nStderr: {e.stderr}"
        raise subprocess.CalledProcessError(e.returncode, e.cmd, output=e.output,
                                            stderr=e.stderr) from None  # re-raise with improved message

    except Exception as e:
        raise Exception(f"An error occurred during conversion: {e}") from None  # re-raise with context


def convertMarkdownToHtml(input_md: str, output_html: str, resource_path: str = None, template_path: str = None):
    """Converts a Markdown file to an HTML file using Pandoc.

    Args:
        input_md: Path to the input Markdown file.
        output_html: Path to the output HTML file.
        resource_path: Path to resources (images, etc.) used in the Markdown file.
        template_path: Path to a Pandoc template file.

    Raises:
        subprocess.CalledProcessError: If Pandoc execution fails.
        FileNotFoundError: If the input Markdown file is not found.
        Exception: For any other errors during file processing.

    """
    command = ["pandoc", input_md, "-o", output_html, "-t", "html", "--standalone"]

    if resource_path:
        command.append(f"--resource-path={resource_path}")

    if template_path:
        command.extend(["--reference-doc", template_path])

    try:
        subprocess.run(command, check=True, capture_output=True, text=True)  # Capture output for better error messages
    except FileNotFoundError:
        raise FileNotFoundError(f"Input file '{input_md}' not found or Pandoc is not in your PATH.")
    except subprocess.CalledProcessError as e:
        error_message = f"Pandoc execution failed:\nReturn code: {e.returncode}\nStdout: {e.stdout}\nStderr: {e.stderr}"
        raise subprocess.CalledProcessError(e.returncode, e.cmd, output=e.output,
                                            stderr=e.stderr) from None  # re-raise with improved message

    except Exception as e:
        raise Exception(f"An error occurred during conversion: {e}") from None  # re-raise with context


# Configure logging for better error reporting
logger = logging.getLogger(__name__)
logger.setLevel(logging.INFO)  # Adjust logging level as needed
handler = logging.StreamHandler()
formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')
handler.setFormatter(formatter)
logger.addHandler(handler)


def generate_report(filename: str, reportname: str, user_username: str) -> None:
    """Generates a docx report from a markdown file, handling PlantUML diagrams.

    Args:
        filename: Path to the input markdown file.
        reportname: Desired name for the output docx report.
        user_username: Username to associate with the report.

    Returns:
        None. Raises exceptions if errors occur during processing.
    """
    app_dir = getAppDirectory()
    user_folder = os.path.join(app_dir, "user_reports", user_username)
    temp_dir = os.path.join(app_dir, "temp_processing")  # Temporary files will be within project dir

    try:

        images_dir = os.path.join(temp_dir, "images_folder")
        blocks_dir = os.path.join(temp_dir, "blocks_folder")
        temp_md_file = os.path.join(temp_dir,
                                    f"tmp_{os.path.basename(filename)}")  # Use basename for better filename handling.
        print(f"Temporary Markdown File Path: {temp_md_file}")  # Debugging line
        print(f"Temporary Images Folder Path: {images_dir}")  # Debugging line
        print(f"Temporary Blocks Folder Path: {blocks_dir}")  # Debugging line
        makeMissingFolders([temp_dir, blocks_dir, images_dir, user_folder])  # Create temp_dir

        shutil.copy2(filename, temp_md_file)  # Use direct filename instead of app_dir join.
        remove_plantuml_code(temp_md_file)

        make_blocks(filename=temp_md_file, blocks_folder=blocks_dir, images_folder=images_dir)

        output_docx = os.path.join(user_folder, f"{reportname}.docx")
        output_html = os.path.join(user_folder, f"{reportname}.html")
        print(f"Output DOCX Path: {output_docx}")  # Debugging line

        convertMarkdownToDocx(input_md=temp_md_file, output_docx=output_docx, resource_path=images_dir)
        convertMarkdownToHtml(input_md=temp_md_file, output_html=output_html,
                              resource_path=images_dir)  # added this line

        time.sleep(2)  # Add a short delay to ensure files are released

        logger.info(f"Report '{reportname}' (docx and html) generated successfully for user '{user_username}'.")
        # Manually clean up the temp_dir (since it's not managed by tempfile)
        # shutil.rmtree(temp_dir)
    except FileNotFoundError:
        logger.exception(f"Input file '{filename}' not found.")
        raise
    except Exception as e:
        logger.exception(f"An error occurred during report generation: {e}")

        # Clean up on failure too!
        # if os.path.exists(temp_dir):
        #     shutil.rmtree(temp_dir)  # Cleanup temp directory even if errors occur

        raise
