import subprocess
import sys


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
        raise subprocess.CalledProcessError(e.returncode, e.cmd, output=e.output, stderr=e.stderr) from None #re-raise with improved message

    except Exception as e:
        raise Exception(f"An error occurred during conversion: {e}") from None #re-raise with context
