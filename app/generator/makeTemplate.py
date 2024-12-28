import subprocess
from jinja2 import Environment, FileSystemLoader
import os
import tempfile

# ... (Existing code remains the same) ...

def generate_and_convert_report(template_path, output_docx_path, output_html_path, data, style_docx_path):
    """Generates a report from a Jinja2 template, fills it with data, and converts to docx and html."""
    try:
        # 1. Render the Jinja2 template
        env = Environment(loader=FileSystemLoader(os.path.dirname(template_path)))
        template = env.get_template(os.path.basename(template_path))
        rendered_markdown = template.render(**data)

        # 2. Write rendered Markdown to a temporary file
        with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False) as temp_md_file:
            temp_md_file.write(rendered_markdown)
            temp_md_filepath = temp_md_file.name

        # 3. Convert Markdown to docx using Pandoc
        subprocess.run(
            [
                "pandoc",
                "--from",
                "markdown",
                "--to",
                "docx",
                "--reference-doc",
                style_docx_path, # Added style path
                "-o",
                output_docx_path,
                temp_md_filepath,
            ],
            check=True,
        )

        # 4. Convert Markdown to html using Pandoc (added)
        subprocess.run(
            [
                "pandoc",
                "--from",
                "markdown",
                "--to",
                "html",
                "--reference-doc",
                style_docx_path, # Added style path
                "-o",
                output_html_path,
                temp_md_filepath,
            ],
            check=True,
        )

        # Clean up temporary file
        os.remove(temp_md_filepath)

    except FileNotFoundError:
        raise FileNotFoundError(f"Template file not found: {template_path}")
    except subprocess.CalledProcessError as e:
        raise RuntimeError(f"Pandoc execution failed: {e}")
    except Exception as e:
        raise RuntimeError(f"An error occurred: {e}")


# Example usage (replace with your actual data and paths)
data = {
    "project_title": "Automated Report Generation",
    "student_name": "Ahmed Murtada Albanna",
    "department": "Computer Science",
    "university": "Sanaa University",
    "date": "October 26, 2024",
    "email": "ahmedalbanna966@gmail.com",
    "chapters": [
        {"title": "Introduction", "content": "This is the introduction chapter..."},
        {"title": "Methodology", "content": "Details about the methodology..."},
    ],
}

template_path = "ReportTemplate.md"  # Path to your Jinja2 template
output_docx_path = "report_output.docx"  # Path for the output docx file
output_html_path = "report_output.html"  # Path for the output html file
style_docx_path = "report_Style.docx" # Path to your style docx file

generate_and_convert_report(template_path, output_docx_path, output_html_path, data,style_docx_path)


