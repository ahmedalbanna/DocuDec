from jinja2 import Environment, FileSystemLoader
import os
import markdown  # For converting markdown to HTML if needed


def generate_report(template_path, output_path, data):
    """Generates a report from a Jinja2 template and data."""
    env = Environment(loader=FileSystemLoader(os.path.dirname(template_path)))
    template = env.get_template(os.path.basename(template_path))


    # Render the template with the provided data
    rendered_report = template.render(**data) #updated code

    # If you need Markdown to HTML conversion:
    # html_report = markdown.markdown(rendered_report, extensions=['tables', 'fenced_code'])  # Add extensions as needed like code highlighting
    # with open(output_path, 'w', encoding='utf-8') as f: #updated code
    #     f.write(html_report)
    with open(output_path, 'w', encoding='utf-8') as f: #updated code
        f.write(rendered_report)

# Example usage (replace with your actual data and paths)
data = {
    "project_title": "Automated Generation of Reports",
    "student_name": "Ahmed Murtada Albanna",
    "department": "Computer Science",
    "university": "Sanaa University",
    "date": "October 26, 2024",
    "email": "ahmedalbanna966@gmail.com",
    # ... Add all other variables from the template ...
    # including any lists or more complex data
    "chapters": [
        {"title": "Introduction", "content": "This is the introduction chapter..."},
        {"title": "Methodology", "content": "Details about the methodology..."},
        # ... more chapters ...
    ],

}




generate_report("G:/Docudec/templates/ReportTemplate.md", "report_output.md", data)