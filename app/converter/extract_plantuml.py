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

