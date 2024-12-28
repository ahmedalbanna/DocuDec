import re


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
