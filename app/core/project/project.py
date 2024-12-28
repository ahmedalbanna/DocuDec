import json
from typing import Union

def format_data(project: str, data: dict) -> None:
    """Formats and prints project data from a JSON file."""

    project_data = data.get(project)
    if not project_data:
        print("No 'project' data found in the file.")
        return

    def format_value(key: str, value: Union[list, dict, str, int]):
        print(f"\n{key.capitalize()}:")
        if isinstance(value, list):
            for item in value:
                print(f"- {item}")
        elif isinstance(value, dict):
            for d_key, d_value in value.items():
                format_value(d_key, d_value)
        else:
            print(value)

    print("Project Data:")
    for key, value in project_data.items():
        format_value(key, value)

with open("project.json", "r") as f:
    format_data( "project", json.load(f))
