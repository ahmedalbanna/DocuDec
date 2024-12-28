import logging
import os
import shutil
import tempfile  # Import tempfile module
import time       # Import time module

from .convert_md_to_docx import convertMarkdownToDocx
from .convert_md_to_html import convertMarkdownToHtml 
from .generate_blocks import make_blocks
from .get_app_dir import getAppDirectory
from .make_missing_folders import makeMissingFolders
from .remove_block_code import remove_plantuml_code

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
    temp_dir = os.path.join(app_dir, "temp_processing") # Temporary files will be within project dir

    try:
        
            images_dir = os.path.join(temp_dir, "images_folder")
            blocks_dir = os.path.join(temp_dir, "blocks_folder")
            temp_md_file = os.path.join(temp_dir, f"tmp_{os.path.basename(filename)}")  # Use basename for better filename handling.
            print(f"Temporary Markdown File Path: {temp_md_file}")  # Debugging line
            print(f"Temporary Images Folder Path: {images_dir}")  # Debugging line
            print(f"Temporary Blocks Folder Path: {blocks_dir}")  # Debugging line
            makeMissingFolders([temp_dir, blocks_dir, images_dir, user_folder]) # Create temp_dir

            shutil.copy2(filename, temp_md_file)  # Use direct filename instead of app_dir join.
            remove_plantuml_code(temp_md_file)

            make_blocks(filename=temp_md_file, blocks_folder=blocks_dir, images_folder=images_dir)

            output_docx = os.path.join(user_folder, f"{reportname}.docx")
            output_html = os.path.join(user_folder, f"{reportname}.html")
            print(f"Output DOCX Path: {output_docx}")  # Debugging line

            convertMarkdownToDocx(input_md=temp_md_file, output_docx=output_docx, resource_path=images_dir)
            convertMarkdownToHtml(input_md=temp_md_file, output_html=output_html, resource_path=images_dir) #added this line

            time.sleep(2)  # Add a short delay to ensure files are released
            
            logger.info(f"Report '{reportname}' (docx and html) generated successfully for user '{user_username}'.")
            # Manually clean up the temp_dir (since it's not managed by tempfile)
            #shutil.rmtree(temp_dir)
    except FileNotFoundError:
        logger.exception(f"Input file '{filename}' not found.")
        raise
    except Exception as e:
        logger.exception(f"An error occurred during report generation: {e}")

        # Clean up on failure too!
        # if os.path.exists(temp_dir):
        #     shutil.rmtree(temp_dir)  # Cleanup temp directory even if errors occur

        
        raise
