import os

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


