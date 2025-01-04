import os
import google.generativeai as genai
from google.ai.generativelanguage_v1beta.types import content
from IPython.display import display, Markdown
import json
from jinja2 import Template
import typing
from typing import Any, Dict, List
from diskcache import Cache
import logging
import shutil

# Configure logging
logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)
cache_dir = ".cache"  # Path to your cache directory

# Check if the cache directory exists
# if os.path.exists(cache_dir):
#     try:
#         shutil.rmtree(cache_dir) # Recursively delete the directory and its contents
#         print(f"Cache directory '{cache_dir}' cleared successfully.")
#     except OSError as e:
#         print(f"Error clearing cache directory: {e}")
# else:
#     print(f"Cache directory '{cache_dir}' not found.")

# Recreate the cache (optional, but usually good practice)
cache = Cache(cache_dir)  # Specify cache directory


# --- Setup Google AI API ---
def configure_google_ai():
    """Configures the Google AI API with the API key from environment variables."""
    # genai.configure(api_key=os.environ.get("GEMINI_API_KEY"))
    genai.configure(api_key="AIzaSyDIlivnAT09Y8D5AIZKU0fnWOXJIY0dP4s")


# --- Reusable Model and Chat Session Creation ---
def create_model(
    model_name: str,
    generation_config: Dict[str, Any],
    system_instruction: str,
    response_mime_type: str = "application/json",
) -> genai.GenerativeModel:
    """
    Creates a GenerativeModel instance with provided configurations.
    Args:
        model_name (str): The name of the model to use.
        generation_config (Dict[str, Any]): Generation configurations for the model.
        system_instruction (str): Instructions to the model as a system.
        response_mime_type (str, optional): The mime type for response. Defaults to "application/json".
    Returns:
        genai.GenerativeModel: A configured GenerativeModel.
    """
    generation_config["response_mime_type"] = response_mime_type
    return genai.GenerativeModel(
        model_name=model_name,
        generation_config=generation_config,
        system_instruction=system_instruction,
    )


def create_chat_session(
    model: genai.GenerativeModel, history: typing.Optional[List] = None
) -> genai.ChatSession:
    """
    Creates a chat session from given model and history.
    Args:
        model (genai.GenerativeModel): The generative model to use for the chat.
        history (typing.Optional[List], optional): Chat history as a list. Defaults to None.
    Returns:
        genai.ChatSession: A chat session instance.
    """
    return model.start_chat(history=history or [])


def format_markdown_from_data(data: dict, indent: str = "  ", level: int = 0) -> str:
    """Formats nested dictionary data into a markdown string.
    Args:
        data (dict): The dictionary to format
        indent (str, optional): The string for indenting items. Defaults to "  ".
        level (int, optional): The level of the heading . Defaults to 0.
    Returns:
       str: A formatted markdown string.
    """

    def format_dict(d: dict, level_indent: str, level: int) -> str:
        md = ""
        for key, value in d.items():
            if value is None:
                continue
            if isinstance(value, dict):
                md += f"{level_indent}{'#'*(level+1)} **{key.replace('_', ' ').title()}**\n"
                md += format_dict(value, level_indent + indent, level + 1)
            elif isinstance(value, list):
                md += f"{level_indent}{'#'*(level+1)} **{key.replace('_', ' ').title()}**\n"
                md += format_list(value, level_indent + indent, level + 1)
            else:
                md += f"{level_indent}- **{key.replace('_', ' ').title()}**: {value}\n"
        return md

    def format_list(lst: list, level_indent: str, level: int) -> str:
        md = ""
        for item in lst:
            if isinstance(item, dict):
                md += format_dict(item, level_indent + "\n- ", level + 1)
            elif isinstance(item, list):
                md += format_list(item, level_indent + indent, level + 1)
            else:
                md += f"{level_indent}- {item}\n"
        return md

    if isinstance(data, dict):  # Correct: Call format_dict if it's a dict
        return format_dict(data, indent, level)
    elif isinstance(data, list):  # Correct: Call format_list if it's a list
        return format_list(data, indent, level)
    else:
        return str(data)  # Handle other data types or log a warning


def print_markdown_report(
    data: dict, title: str = "", indent: str = "  ", level: int = 0
) -> None:
    """
    Generates a formatted Markdown report based on nested JSON data.

    Args:
        data (dict): The data in json or dictionary
        title (str, optional): The title of the markdown document. Defaults to "".
        indent (str, optional): The string for indenting items. Defaults to "  ".
        level (int, optional): The level of the heading . Defaults to 0.
    Returns:
        None: This function prints to the display.
    """
    md_report = f"{'#'* (level+1)} {title}\n\n" if title else ""
    md_report += format_markdown_from_data(data, indent, level)
    display(Markdown(md_report))


@cache.memoize()  # Decorate functions to cache their results
def generate_project_titles(
    project_title: str,
) -> Dict[str, Any]:
    """
    Generates alternative project titles using the Gemini model.
    Args:
        project_title (str): The project title to generate alternatives for.
    Returns:
        Dict[str, Any]: A dictionary of alternative project titles.
    """
    configure_google_ai()
    generation_config_title = {
        "temperature": 1.5,
        "top_p": 0.95,
        "top_k": 40,
        "max_output_tokens": 500,
        "response_schema": content.Schema(
            type=content.Type.OBJECT,
            description="Schema for analyzing and suggesting alternative titles for a project.",
            enum=[],
            required=["originalTitle", "recommendation", "alternativeTitles"],
            properties={
                "originalTitle": content.Schema(
                    type=content.Type.STRING,
                ),
                "recommendation": content.Schema(
                    type=content.Type.STRING,
                ),
                "alternativeTitles": content.Schema(
                    type=content.Type.OBJECT,
                    description="Suggested alternative titles categorized by focus areas.",
                    enum=[],
                    required=[
                        "focusingOnAutomation",
                        "focusingOnInnovationOrProblemSolving",
                        "focusingOnReports",
                        "moreConciseTitles",
                        "moreSpecificAndTechnicalTitles",
                    ],
                    properties={
                        "focusingOnAutomation": content.Schema(
                            type=content.Type.ARRAY,
                            items=content.Schema(
                                type=content.Type.STRING,
                            ),
                        ),
                        "focusingOnInnovationOrProblemSolving": content.Schema(
                            type=content.Type.ARRAY,
                            items=content.Schema(
                                type=content.Type.STRING,
                            ),
                        ),
                        "focusingOnReports": content.Schema(
                            type=content.Type.ARRAY,
                            items=content.Schema(
                                type=content.Type.STRING,
                            ),
                        ),
                        "moreConciseTitles": content.Schema(
                            type=content.Type.ARRAY,
                            items=content.Schema(
                                type=content.Type.STRING,
                            ),
                        ),
                        "moreSpecificAndTechnicalTitles": content.Schema(
                            type=content.Type.ARRAY,
                            items=content.Schema(
                                type=content.Type.STRING,
                            ),
                        ),
                    },
                ),
            },
        ),
    }

    model_title = create_model(
        "gemini-2.0-flash-exp",
        generation_config_title,
        system_instruction="## Generate Project Title\n\n**Preparation Steps**\n\n1. **Review the Project Scope:**\n   - Identify key elements and themes.\n   - Note any specific terminology used.\n2. **Analyze the Project Objectives:**\n   - List primary goals and outcomes.\n   - Understand the purpose and intended impact.\n3. **Determine the Field of Study:**\n   - Identify relevant academic or industry terms.\n   - Consider any interdisciplinary aspects.\n\n**Drafting and Evaluation**\n\n1. **Draft Potential Titles:**\n   - Combine key elements from the scope, objectives, and field.\n   - Ensure clarity and precision in wording.\n2. **Evaluate Titles for Accuracy and Relevance:**\n   - Check alignment with project scope and objectives.\n   - Ensure it reflects the significance of the project.\n3. **Select the Most Suitable Project Title:**\n   - Get feedback from stakeholders if necessary.\n   - Finalize the title based on feedback and evaluation.\n",
    )

    chat_session_title = create_chat_session(model_title)
    try:
        response_title = chat_session_title.send_message(
            f"Project Title: {project_title}"
        )
        titles = json.loads(response_title.text)
        return titles
    except (json.JSONDecodeError, AttributeError) as e:
        logging.error(f"Error generating titles: {e}")
        return {}


def get_selected_title(titles: Dict[str, Any]) -> str:
    """
    Allows the user to select a project title from the generated alternatives.
    Args:
        titles (Dict[str, Any]): A dictionary of project titles.
    Returns:
        str: A selected project title.
    """
    if not titles or "alternativeTitles" not in titles:
        return ""
    print(f"Original Title: {titles['originalTitle']}")
    all_titles = []
    for category, titles_list in titles["alternativeTitles"].items():
        all_titles.extend(titles_list)

    print("Available Titles:")
    for i, title in enumerate(all_titles):
        print(f"{i+1}. {title}")

    while True:
        try:
            choice = int(input(f"Enter your choice (1-{len(all_titles)}): "))
            if 1 <= choice <= len(all_titles):
                selected_title = all_titles[choice - 1]
                print(f"You selected: {selected_title}")
                return selected_title
            else:
                print("Invalid choice. Please enter a number within the valid range.")
        except ValueError:
            print("Invalid input. Please enter a number.")


@cache.memoize()
def generate_project_info(
    selected_title: str,
) -> Dict[str, Any]:
    """
    Generates project information using the Gemini model.
    Args:
        selected_title (str): The selected project title.
    Returns:
         Dict[str, Any]: A dictionary of project information.
    """
    configure_google_ai()
    generation_config_projectInfo = {
        "temperature": 1,
        "top_p": 0.95,
        "top_k": 40,
        "max_output_tokens": 1500,
        "response_schema": content.Schema(
            type=content.Type.OBJECT,
            enum=[],
            required=["projectInformation"],
            properties={
                "projectInformation": content.Schema(
                    type=content.Type.OBJECT,
                    enum=[],
                    required=[
                        "projectTitle",
                        "projectScope",
                        "fieldOfStudy",
                        "targetAudience",
                    ],
                    properties={
                        "projectTitle": content.Schema(
                            type=content.Type.STRING,
                        ),
                        "projectScope": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=[
                                "description",
                                "objectivesAndGoals",
                                "boundaries",
                                "stakeholderIdentification",
                                "timeline",
                                "resourceAllocation",
                                "riskAssessment",
                                "approvalProcess",
                            ],
                            properties={
                                "description": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "objectivesAndGoals": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                                "boundaries": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "stakeholderIdentification": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                                "timeline": content.Schema(
                                    type=content.Type.OBJECT,
                                    enum=[],
                                    required=["start", "end", "milestones"],
                                    properties={
                                        "start": content.Schema(
                                            type=content.Type.STRING,
                                        ),
                                        "end": content.Schema(
                                            type=content.Type.STRING,
                                        ),
                                        "milestones": content.Schema(
                                            type=content.Type.ARRAY,
                                            items=content.Schema(
                                                type=content.Type.STRING,
                                            ),
                                        ),
                                    },
                                ),
                                "resourceAllocation": content.Schema(
                                    type=content.Type.OBJECT,
                                    enum=[],
                                    required=["teamResponsibilities"],
                                    properties={
                                        "teamResponsibilities": content.Schema(
                                            type=content.Type.ARRAY,
                                            items=content.Schema(
                                                type=content.Type.STRING,
                                            ),
                                        ),
                                    },
                                ),
                                "riskAssessment": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "approvalProcess": content.Schema(
                                    type=content.Type.STRING,
                                ),
                            },
                        ),
                        "fieldOfStudy": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["academicField", "justification"],
                            properties={
                                "academicField": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "justification": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                            },
                        ),
                        "targetAudience": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=[
                                "marketResearch",
                                "demographics",
                                "psychographics",
                                "audienceFeedback",
                                "customerDataAnalysis",
                                "audiencePersonas",
                            ],
                            properties={
                                "marketResearch": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "demographics": content.Schema(
                                    type=content.Type.OBJECT,
                                    enum=[],
                                    required=["age", "gender", "location"],
                                    properties={
                                        "age": content.Schema(
                                            type=content.Type.STRING,
                                        ),
                                        "gender": content.Schema(
                                            type=content.Type.STRING,
                                        ),
                                        "location": content.Schema(
                                            type=content.Type.STRING,
                                        ),
                                    },
                                ),
                                "psychographics": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                                "audienceFeedback": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                                "customerDataAnalysis": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "audiencePersonas": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                            },
                        ),
                    },
                ),
            },
        ),
    }
    model_projectInfo = create_model(
        "gemini-2.0-flash-exp",
        generation_config_projectInfo,
        system_instruction="## **1. Project Information**\n\n**1.1 Project Title**\n\n- Clearly state the project title.\n\n**1.2 Project Scope**\n\n- Provide a concise description of the project's scope and objectives. Include:\n  - Defined objectives and key goals\n  - Project boundaries (inclusions and exclusions)\n  - Stakeholder identification (primary and secondary)\n  - Developed timeline with start/end dates and milestones\n  - Resource allocation and team responsibilities\n  - Risk assessment and mitigation strategies\n  - Approval process and sign-off details\n\n**1.3 Field of Study**\n\n- Indicate the academic field relevant to the project. Justify your selection by:\n  - Researching common fields related to your topic\n  - Consulting experts or advisors\n\n**1.4 Target Audience**\n\n- Identify the intended audience by:\n  - Conducting market research, including industry trends and competitor analysis\n  - Defining demographics (age, gender, location, etc.)\n  - Identifying psychographics (interests, values, lifestyle)\n  - Gathering audience feedback through surveys and focus groups\n  - Analyzing existing customer data\n  - Creating and validating audience personas\n",
    )

    chat_session_projectInfo = create_chat_session(model_projectInfo)
    try:
        response_projectInfo = chat_session_projectInfo.send_message(
            f"Project Title: {selected_title}"
        )
        project_info = json.loads(response_projectInfo.text)
        return project_info
    except (json.JSONDecodeError, AttributeError) as e:
        logging.error(f"Error generating project information: {e}")
        return {}


@cache.memoize()
def generate_project_content(
    project_info: Dict[str, Any],
) -> Dict[str, Any]:
    """
    Generates project content using the Gemini model.

    Args:
        project_info (Dict[str, Any]): The project information dictionary

    Returns:
        Dict[str, Any]: A dictionary of project content.
    """
    configure_google_ai()
    generation_config_projectContent = {
        "temperature": 1,
        "top_p": 0.95,
        "top_k": 40,
        "max_output_tokens": 2500,
        "response_schema": content.Schema(
            type=content.Type.OBJECT,
            enum=[],
            required=["projectContent"],
            properties={
                "projectContent": content.Schema(
                    type=content.Type.OBJECT,
                    enum=[],
                    required=[
                        "problemStatement",
                        "researchQuestions",
                        "literatureReview",
                        "methodology",
                        "systemDesign",
                        "implementation",
                        "testingAndValidation",
                        "resultsAndAnalysis",
                        "futureWork",
                        "contributions",
                    ],
                    properties={
                        "problemStatement": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["description", "steps"],
                            properties={
                                "description": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "steps": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                            },
                        ),
                        "researchQuestions": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["description", "steps"],
                            properties={
                                "description": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "steps": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                            },
                        ),
                        "literatureReview": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["description", "steps"],
                            properties={
                                "description": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "steps": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                            },
                        ),
                        "methodology": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["description", "steps"],
                            properties={
                                "description": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "steps": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                            },
                        ),
                        "systemDesign": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["description", "steps"],
                            properties={
                                "description": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "steps": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                            },
                        ),
                        "implementation": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["description", "steps"],
                            properties={
                                "description": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "steps": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                            },
                        ),
                        "testingAndValidation": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["description", "steps"],
                            properties={
                                "description": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "steps": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                            },
                        ),
                        "resultsAndAnalysis": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["description", "steps"],
                            properties={
                                "description": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "steps": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                            },
                        ),
                        "futureWork": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["description", "steps"],
                            properties={
                                "description": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "steps": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                            },
                        ),
                        "contributions": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["description", "steps"],
                            properties={
                                "description": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "steps": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                            },
                        ),
                    },
                ),
            },
        ),
    }

    model_projectContent = create_model(
        "gemini-2.0-flash-exp",
        generation_config_projectContent,
        system_instruction="## **2. Project Content**\n\n**2.1 Problem Statement**\n\n- Clearly outline the problem, including context and significance. Steps include:\n  - Background research and data collection\n  - Scope definition\n  - Impact analysis\n  - Problem statement formulation and validation\n\n**2.2 Research Questions**\n\n- List specific research questions by:\n  - Breaking down the main topic into clear, measurable questions\n  - Aligning questions with research objectives\n  - Prioritizing based on importance and feasibility\n\n**2.3 Literature Review**\n\n- Summarize key findings from existing research. Steps include:\n  - Identifying and organizing relevant papers\n  - Analyzing methodologies, technologies, and research gaps\n  - Summarizing findings and discussing implications\n\n**2.4 Methodology**\n\n- Describe the approach and methods used in designing, implementing, and testing:\n  - Project design and architecture\n  - Implementation details, including core features\n  - Testing strategy, including test cases and scenarios\n\n**2.5 System Design**\n\n- Explain the architecture, components, data structures, and algorithms:\n  - Requirements gathering\n  - Detailed component and algorithm design\n  - Design review and documentation\n\n**2.6 Implementation**\n\n- Detail programming languages and techniques, including:\n  - Core functionality and additional features\n  - Testing and optimization\n  - Documentation and code review\n\n**2.7 Testing and Validation**\n\n- Outline the testing process, including:\n  - Test planning and creation\n  - Execution and analysis\n  - Issue reporting and documentation\n\n**2.8 Results and Analysis**\n\n- Present and analyze key findings:\n  - Data compilation and summarization\n  - Visual representation and analysis\n  - Discussion on limitations and further research\n\n**2.9 Future Work**\n\n- Discuss potential enhancements and applications, including:\n  - Technology integration\n  - Roadmap for improvements\n  - Scalability, performance, and user experience\n\n**2.10 Contributions**\n\n- Highlight the project's contributions to the field by:\n  - Reviewing objectives and outcomes\n  - Gathering supporting data\n  - Drafting a summary of contributions\n",
    )

    try:
        response_projectContent = model_projectContent.generate_content(
            json.dumps(project_info)
        )
        project_content = json.loads(response_projectContent.text)
        return project_content
    except (json.JSONDecodeError, AttributeError) as e:
        logging.error(f"Error generating project content: {e}")
        return {}


@cache.memoize()
def generate_project_structure(
    project_info: Dict[str, Any],
) -> Dict[str, Any]:
    """
    Generates project structure using the Gemini model.

    Args:
      project_info (Dict[str, Any]): The project information dictionary.

    Returns:
      Dict[str, Any]: A dictionary of the project structure.
    """
    configure_google_ai()
    generation_config_structure = {
        "temperature": 0.95,
        "top_p": 0.95,
        "top_k": 40,
        "max_output_tokens": 1500,
        "response_schema": content.Schema(
            type=content.Type.OBJECT,
            enum=[],
            required=["structureAndFormat"],
            properties={
                "structureAndFormat": content.Schema(
                    type=content.Type.OBJECT,
                    enum=[],
                    required=["chapterStructure", "targetLength", "visualAids"],
                    properties={
                        "chapterStructure": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["chapters", "description"],
                            properties={
                                "chapters": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                                "description": content.Schema(
                                    type=content.Type.STRING,
                                ),
                            },
                        ),
                        "targetLength": content.Schema(
                            type=content.Type.STRING,
                        ),
                        "visualAids": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["diagram"],
                            properties={
                                "diagram": content.Schema(
                                    type=content.Type.ARRAY,
                                    items=content.Schema(
                                        type=content.Type.STRING,
                                    ),
                                ),
                            },
                        ),
                    },
                ),
            },
        ),
    }

    model_structure = create_model(
        "gemini-2.0-flash-exp",
        generation_config_structure,
        system_instruction="## **3. Structure and Format**\n\n**3.1 Target Length**\n\n- Specify the report length (e.g., 10-15 pages).\n\n**3.2 Chapter Structure**\n\n- Outline the report structure with chapter headings and sub-sections including:\n  - Introduction\n  - Literature Review\n  - Methodology\n  - Results\n  - Discussion\n  - Conclusion\n  - References\n  - Appendices\n\n**3.3 Visual Aids**\n\n- Specify desired visual aids (diagrams, charts, tables) ensuring clarity and accuracy.\n",
    )
    try:
        response_structure = model_structure.generate_content(
            json.dumps(
                {
                    "projectInformation": project_info,
                }
            )
        )
        project_structure = json.loads(response_structure.text)
        return project_structure
    except (json.JSONDecodeError, AttributeError) as e:
        logging.error(f"Error generating project structure: {e}")
        return {}


@cache.memoize()
def generate_additional_instructions(
    project_info: Dict[str, Any],
    project_content: Dict[str, Any],
    project_structure: Dict[str, Any],
) -> Dict[str, Any]:
    """
    Generates additional instructions using the Gemini model.

    Args:
      project_info (Dict[str, Any]): The project information dictionary.
      project_content (Dict[str, Any]): The project content dictionary.
      project_structure (Dict[str, Any]): The project structure dictionary.

    Returns:
      Dict[str, Any]: A dictionary of additional instructions.
    """
    configure_google_ai()
    generation_config_instructions = {
        "temperature": 0.95,
        "top_p": 0.95,
        "top_k": 40,
        "max_output_tokens": 2891,
        "response_schema": content.Schema(
            type=content.Type.OBJECT,
            enum=[],
            required=["additionalInstructions"],
            properties={
                "additionalInstructions": content.Schema(
                    type=content.Type.OBJECT,
                    enum=[],
                    required=["constraints", "keywords", "toneAndStyle"],
                    properties={
                        "constraints": content.Schema(
                            type=content.Type.STRING,
                        ),
                        "keywords": content.Schema(
                            type=content.Type.ARRAY,
                            items=content.Schema(
                                type=content.Type.STRING,
                            ),
                        ),
                        "toneAndStyle": content.Schema(
                            type=content.Type.ARRAY,
                            items=content.Schema(
                                type=content.Type.STRING,
                            ),
                        ),
                    },
                ),
            },
        ),
    }
    model_instructions = create_model(
        "gemini-2.0-flash-exp",
        generation_config_instructions,
        system_instruction="## **4. Additional Instructions**\n\n**4.1 Tone and Style**\n\n- Indicate the desired tone (formal, informal, academic) and style.\n\n**4.2 Keywords**\n\n- Include specific keywords or phrases to focus on. Optimize content accordingly.\n\n**4.3 Constraints**\n\n- Specify any constraints like word count, formats, or required elements.\n\nThis comprehensive structure ensures a logical flow from identifying project details to documenting and structuring the report, along with considerations for content, presentation, and additional requirements.\n",
    )
    try:
        response_instructions = model_instructions.generate_content(
            json.dumps(
                {
                    "projectInformation": project_info,
                    "projectContent": project_content,
                    "structureAndFormat": project_structure,
                }
            )
        )
        project_instructions = json.loads(response_instructions.text)
        return project_instructions
    except (json.JSONDecodeError, AttributeError) as e:
        logging.error(f"Error generating project instructions: {e}")
        return {}


@cache.memoize()
def generate_abstract(
    project_info: Dict[str, Any],
    project_content: Dict[str, Any],
    project_structure: Dict[str, Any],
    project_instructions: Dict[str, Any],
) -> Dict[str, Any]:
    """
    Generates the project abstract using the Gemini model.

    Args:
        project_info (Dict[str, Any]): The project information dictionary.
        project_content (Dict[str, Any]): The project content dictionary.
        project_structure (Dict[str, Any]): The project structure dictionary.
        project_instructions (Dict[str, Any]): The project instructions dictionary.

    Returns:
        Dict[str, Any]: A dictionary containing the generated project abstract.
    """
    configure_google_ai()
    generation_config_abstract = {
        "temperature": 1.5,
        "top_p": 0.95,
        "top_k": 40,
        "max_output_tokens": 8192,
        "response_schema": content.Schema(
            type=content.Type.OBJECT,
            enum=[],
            required=["abstract"],
            properties={
                "abstract": content.Schema(
                    type=content.Type.STRING,
                )
            },
        ),
    }
    model_abstract = create_model(
        "gemini-2.0-flash-exp",
        generation_config_abstract,
        system_instruction="## I. Abstract\n\nThis section contains the abstract, which should be a minimum of 500 words. It summarizes the project, including the problem being addressed, the proposed solution, and the implementation plan.\n\n- [ ] Identify the main problem or issue the project addresses\n  - [ ] Gather relevant data or research about the problem\n  - [ ] Summarize the importance and impact of the problem\n- [ ] Outline the proposed solution\n  - [ ] Describe how the solution addresses the problem\n  - [ ] Highlight any innovative aspects of the solution\n- [ ] Detail the implementation plan for the solution\n  - [ ] Identify key steps and strategies for implementation\n  - [ ] List resources and tools required for implementation\n  - [ ] Outline a timeline for implementation phases\n- [ ] Draft the abstract\n  - [ ] Write an introductory sentence about the project\n  - [ ] Summarize the problem and its significance in 2-3 sentences\n  - [ ] Describe the proposed solution in 2-3 sentences\n  - [ ] Explain the implementation plan briefly in 2-3 sentences\n  - [ ] Conclude with a sentence on the expected impact or outcome\n- [ ] Review and refine the abstract\n  - [ ] Check for clarity and conciseness\n  - [ ] Ensure the abstract is at least 500 words\n  - [ ] Edit for grammar and punctuation\n  - [ ] Seek feedback from peers or advisors and make necessary revisions\n",
    )
    try:
        response_abstract = model_abstract.generate_content(
            json.dumps(
                {
                    "projectInformation": project_info,
                    "projectContent": project_content,
                    "structureAndFormat": project_structure,
                    "additionalInstructions": project_instructions,
                }
            )
        )
        project_abstract = json.loads(response_abstract.text)
        return project_abstract
    except (json.JSONDecodeError, AttributeError) as e:
        logging.error(f"Error generating project abstract: {e}")
        return {}


@cache.memoize()
def generate_project_details(
    project_info: Dict[str, Any],
    project_content: Dict[str, Any],
    project_structure: Dict[str, Any],
    project_instructions: Dict[str, Any],
    project_abstract: Dict[str, Any],
) -> Dict[str, Any]:
    """
     Generates project details using the Gemini model.

    Args:
        project_info (Dict[str, Any]): The project information dictionary.
        project_content (Dict[str, Any]): The project content dictionary.
        project_structure (Dict[str, Any]): The project structure dictionary.
        project_instructions (Dict[str, Any]): The project instructions dictionary.
        project_abstract (Dict[str, Any]): The project abstract dictionary.

    Returns:
       Dict[str, Any]: A dictionary of the project details.
    """
    configure_google_ai()
    generation_config_projectDetails = {
        "temperature": 1,
        "top_p": 0.95,
        "top_k": 40,
        "max_output_tokens": 8192,
        "response_schema": content.Schema(
            type=content.Type.OBJECT,
            enum=[],
            required=["projectDetails"],
            properties={
                "projectDetails": content.Schema(
                    type=content.Type.OBJECT,
                    enum=[],
                    required=[
                        "overview",
                        "projectPlanning",
                        "requirementsGathering",
                        "hardwareRequirements",
                        "softwareRequirements",
                        "designPhase",
                        "implementationPhase",
                        "testingPhase",
                        "deploymentPhase",
                        "maintenanceAndSupport",
                        "documentation",
                    ],
                    properties={
                        "overview": content.Schema(
                            type=content.Type.STRING,
                        ),
                        "projectPlanning": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=[
                                "defineScope",
                                "establishTimeline",
                                "identifyStakeholders",
                                "allocateResources",
                            ],
                            properties={
                                "defineScope": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "establishTimeline": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "identifyStakeholders": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "allocateResources": content.Schema(
                                    type=content.Type.STRING,
                                ),
                            },
                        ),
                        "requirementsGathering": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=[
                                "conductInterviews",
                                "reviewDocumentation",
                                "compileUserRequirements",
                            ],
                            properties={
                                "conductInterviews": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "reviewDocumentation": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "compileUserRequirements": content.Schema(
                                    type=content.Type.STRING,
                                ),
                            },
                        ),
                        "hardwareRequirements": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=[
                                "computingPower",
                                "peripheralDevices",
                                "networkConnectivity",
                                "scalability",
                            ],
                            properties={
                                "computingPower": content.Schema(
                                    type=content.Type.OBJECT,
                                    enum=[],
                                    required=["cpu", "ram", "storage"],
                                    properties={
                                        "cpu": content.Schema(
                                            type=content.Type.STRING,
                                        ),
                                        "ram": content.Schema(
                                            type=content.Type.STRING,
                                        ),
                                        "storage": content.Schema(
                                            type=content.Type.STRING,
                                        ),
                                    },
                                ),
                                "peripheralDevices": content.Schema(
                                    type=content.Type.OBJECT,
                                    enum=[],
                                    required=["inputDevices", "outputDevices"],
                                    properties={
                                        "inputDevices": content.Schema(
                                            type=content.Type.STRING,
                                        ),
                                        "outputDevices": content.Schema(
                                            type=content.Type.STRING,
                                        ),
                                    },
                                ),
                                "networkConnectivity": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "scalability": content.Schema(
                                    type=content.Type.STRING,
                                ),
                            },
                        ),
                        "softwareRequirements": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=[
                                "operatingSystem",
                                "softwareApplications",
                                "databaseRequirements",
                                "securitySoftwareNeeds",
                            ],
                            properties={
                                "operatingSystem": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "softwareApplications": content.Schema(
                                    type=content.Type.OBJECT,
                                    enum=[],
                                    required=[
                                        "listApplications",
                                        "versionCompatibility",
                                    ],
                                    properties={
                                        "listApplications": content.Schema(
                                            type=content.Type.STRING,
                                        ),
                                        "versionCompatibility": content.Schema(
                                            type=content.Type.STRING,
                                        ),
                                    },
                                ),
                                "databaseRequirements": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "securitySoftwareNeeds": content.Schema(
                                    type=content.Type.STRING,
                                ),
                            },
                        ),
                        "designPhase": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=[
                                "systemArchitectureDiagrams",
                                "userInterfaceMockups",
                                "databaseSchema",
                            ],
                            properties={
                                "systemArchitectureDiagrams": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "userInterfaceMockups": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "databaseSchema": content.Schema(
                                    type=content.Type.STRING,
                                ),
                            },
                        ),
                        "implementationPhase": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=[
                                "hardwareSetup",
                                "softwareInstallation",
                                "codeDevelopment",
                            ],
                            properties={
                                "hardwareSetup": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "softwareInstallation": content.Schema(
                                    type=content.Type.OBJECT,
                                    enum=[],
                                    required=["installOS", "installApplications"],
                                    properties={
                                        "installOS": content.Schema(
                                            type=content.Type.STRING,
                                        ),
                                        "installApplications": content.Schema(
                                            type=content.Type.STRING,
                                        ),
                                    },
                                ),
                                "codeDevelopment": content.Schema(
                                    type=content.Type.STRING,
                                ),
                            },
                        ),
                        "testingPhase": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=[
                                "createTestPlan",
                                "unitTesting",
                                "integrationTesting",
                                "userAcceptanceTesting",
                            ],
                            properties={
                                "createTestPlan": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "unitTesting": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "integrationTesting": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "userAcceptanceTesting": content.Schema(
                                    type=content.Type.STRING,
                                ),
                            },
                        ),
                        "deploymentPhase": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["deploymentPlan", "userTraining"],
                            properties={
                                "deploymentPlan": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "userTraining": content.Schema(
                                    type=content.Type.STRING,
                                ),
                            },
                        ),
                        "maintenanceAndSupport": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["maintenancePlan", "supportServices"],
                            properties={
                                "maintenancePlan": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "supportServices": content.Schema(
                                    type=content.Type.STRING,
                                ),
                            },
                        ),
                        "documentation": content.Schema(
                            type=content.Type.OBJECT,
                            enum=[],
                            required=["userManual", "technicalDocumentation"],
                            properties={
                                "userManual": content.Schema(
                                    type=content.Type.STRING,
                                ),
                                "technicalDocumentation": content.Schema(
                                    type=content.Type.STRING,
                                ),
                            },
                        ),
                    },
                ),
            },
        ),
    }
    model_projectDetails = create_model(
        "gemini-2.0-flash-exp",
        generation_config_projectDetails,
        system_instruction="## **5. Project Details**\n\n**5.1 Overview**\n\n- Provide a brief overview of the project, including its purpose and objectives.\n\n**5.2 Project Planning**\n\n- Outline the project planning phase with details on scope, timeline, stakeholders, and resource allocation.\n\n**5.3 Requirements Gathering**\n\n- Describe the process of gathering user requirements through interviews, documentation review, and compilation.\n\n**5.4 Hardware Requirements**\n\n- Specify the hardware requirements, including computing power, peripheral devices, network connectivity, and scalability.\n\n**5.5 Software Requirements**\n\n- Detail the software requirements, such as the operating system, software applications, database needs, and security software.\n\n**5.6 Design Phase**\n\n- Present the design phase with system architecture diagrams, user interface mockups, and database schema.\n\n**5.7 Implementation Phase**\n\n- Describe the implementation phase, including hardware setup, software installation, and code development.\n\n**5.8 Testing Phase**\n\n- Explain the testing phase, covering test plan creation, unit testing, integration testing, and user acceptance testing.\n\n**5.9 Deployment Phase**\n\n- Detail the deployment phase, including the deployment plan and user training.\n\n**5.10 Maintenance and Support**\n\n- Outline the maintenance plan and support services for the project.\n\n**5.11 Documentation**\n\n- Provide details on user manuals and technical documentation for the project.\n",
    )

    try:
        response_projectDetails = model_projectDetails.generate_content(
            json.dumps(
                {
                    "projectInformation": project_info,
                    "projectContent": project_content,
                    "structureAndFormat": project_structure,
                    "additionalInstructions": project_instructions,
                    "abstract": project_abstract,
                }
            )
        )
        project_details = json.loads(response_projectDetails.text)
        return project_details
    except (json.JSONDecodeError, AttributeError) as e:
        logging.error(f"Error generating project details: {e}")
        return {}


@cache.memoize()
def generate_conclusion(
    project_info: Dict[str, Any],
    project_content: Dict[str, Any],
    project_structure: Dict[str, Any],
    project_instructions: Dict[str, Any],
    project_abstract: Dict[str, Any],
    project_details: Dict[str, Any],
) -> Dict[str, Any]:
    """
    Generates the project conclusion using the Gemini model.

    Args:
        project_info (Dict[str, Any]): The project information dictionary.
        project_content (Dict[str, Any]): The project content dictionary.
        project_structure (Dict[str, Any]): The project structure dictionary.
        project_instructions (Dict[str, Any]): The project instructions dictionary.
        project_abstract (Dict[str, Any]): The project abstract dictionary.
        project_details (Dict[str, Any]): The project details dictionary.

    Returns:
        Dict[str, Any]: A dictionary containing the generated conclusion.
    """
    configure_google_ai()
    generation_config_conclusion = {
        "temperature": 1,
        "top_p": 0.95,
        "top_k": 40,
        "max_output_tokens": 8192,
        "response_schema": content.Schema(
            type=content.Type.OBJECT,
            properties={
                "conclusion": content.Schema(
                    type=content.Type.OBJECT,
                    enum=[],
                    required=[
                        "summaryOfFindings",
                        "achievements",
                        "lessonsLearned",
                        "futureDirections",
                        "finalThoughts",
                    ],
                    properties={
                        "summaryOfFindings": content.Schema(
                            type=content.Type.STRING,
                        ),
                        "achievements": content.Schema(
                            type=content.Type.STRING,
                        ),
                        "lessonsLearned": content.Schema(
                            type=content.Type.STRING,
                        ),
                        "futureDirections": content.Schema(
                            type=content.Type.STRING,
                        ),
                        "finalThoughts": content.Schema(
                            type=content.Type.STRING,
                        ),
                    },
                ),
            },
        ),
        "response_mime_type": "application/json",
    }
    model_conclusion = create_model(
        "gemini-2.0-flash-exp",
        generation_config_conclusion,
        system_instruction="## **6. Conclusion**\n\n**6.1 Summary of Findings**\n\n- Summarize the key findings and results of the project.\n\n**6.2 Achievements**\n\n- Highlight the achievements and contributions of the project.\n\n**6.3 Lessons Learned**\n\n- Discuss the lessons learned during the project development.\n\n**6.4 Future Directions**\n\n- Propose potential future directions or areas for improvement.\n\n**6.5 Final Thoughts**\n\n- Conclude with final thoughts, reflections, and acknowledgments.\n",
    )
    try:
        response_conclusion = model_conclusion.generate_content(
            json.dumps(
                {
                    "projectInformation": project_info,
                    "projectContent": project_content,
                    "structureAndFormat": project_structure,
                    "additionalInstructions": project_instructions,
                    "abstract": project_abstract,
                    "projectDetails": project_details,
                }
            )
        )
        project_conclusion = json.loads(response_conclusion.text)
        return project_conclusion
    except (json.JSONDecodeError, AttributeError) as e:
        logging.error(f"Error generating project conclusion: {e}")
        return {}


@cache.memoize()
def generate_references(
    project_info: Dict[str, Any],
    project_content: Dict[str, Any],
    project_structure: Dict[str, Any],
    project_instructions: Dict[str, Any],
    project_abstract: Dict[str, Any],
    project_details: Dict[str, Any],
    project_conclusion: Dict[str, Any],
) -> Dict[str, Any]:
    """
    Generates project references using the Gemini model.
    Args:
        project_info (Dict[str, Any]): The project information dictionary.
        project_content (Dict[str, Any]): The project content dictionary.
        project_structure (Dict[str, Any]): The project structure dictionary.
        project_instructions (Dict[str, Any]): The project instructions dictionary.
        project_abstract (Dict[str, Any]): The project abstract dictionary.
        project_details (Dict[str, Any]): The project details dictionary.
        project_conclusion (Dict[str, Any]): The project conclusion dictionary.
    Returns:
        Dict[str, Any]: A dictionary containing the generated references.
    """
    configure_google_ai()
    generation_config_references = {
        "temperature": 1,
        "top_p": 0.95,
        "top_k": 40,
        "max_output_tokens": 8192,
        "response_schema": content.Schema(
            type=content.Type.OBJECT,
            enum=[],
            required=["references"],
            properties={
                "references": content.Schema(
                    type=content.Type.ARRAY,
                    items=content.Schema(
                        type=content.Type.OBJECT,
                        enum=[],
                        required=["authors", "title", "year", "publisher", "url"],
                        properties={
                            "authors": content.Schema(
                                type=content.Type.STRING,
                            ),
                            "title": content.Schema(
                                type=content.Type.STRING,
                            ),
                            "year": content.Schema(
                                type=content.Type.STRING,
                            ),
                            "publisher": content.Schema(
                                type=content.Type.STRING,
                            ),
                            "url": content.Schema(
                                type=content.Type.STRING,
                            ),
                        },
                    ),
                ),
            },
        ),
        "response_mime_type": "application/json",
    }
    model_references = create_model(
        "gemini-2.0-flash-exp",
        generation_config_references,
        system_instruction="## **7. References**\n\n- List all the references used in the project, including academic papers, books, articles, and online resources.\n- Follow the appropriate citation style (APA, MLA, Chicago, etc.) for consistency and accuracy.\n",
    )

    try:
        response_references = model_references.generate_content(
            json.dumps(
                {
                    "projectInformation": project_info,
                    "projectContent": project_content,
                    "structureAndFormat": project_structure,
                    "additionalInstructions": project_instructions,
                    "abstract": project_abstract,
                    "projectDetails": project_details,
                    "conclusion": project_conclusion,
                }
            )
        )
        project_references = json.loads(response_references.text)
        return project_references
    except (json.JSONDecodeError, AttributeError) as e:
        logging.error(f"Error generating project references: {e}")
        return {}


@cache.memoize()
def generate_appendices(
    project_info: Dict[str, Any],
    project_content: Dict[str, Any],
    project_structure: Dict[str, Any],
    project_instructions: Dict[str, Any],
    project_abstract: Dict[str, Any],
    project_details: Dict[str, Any],
    project_conclusion: Dict[str, Any],
    project_references: Dict[str, Any],
) -> Dict[str, Any]:
    """
    Generates project appendices using the Gemini model.

    Args:
        project_info (Dict[str, Any]): The project information dictionary.
        project_content (Dict[str, Any]): The project content dictionary.
        project_structure (Dict[str, Any]): The project structure dictionary.
        project_instructions (Dict[str, Any]): The project instructions dictionary.
        project_abstract (Dict[str, Any]): The project abstract dictionary.
        project_details (Dict[str, Any]): The project details dictionary.
        project_conclusion (Dict[str, Any]): The project conclusion dictionary.
        project_references (Dict[str, Any]): The project references dictionary.

    Returns:
      Dict[str, Any]: A dictionary containing the generated appendices.
    """
    configure_google_ai()
    generation_config_appendices = {
        "temperature": 1,
        "top_p": 0.95,
        "top_k": 40,
        "max_output_tokens": 8192,
        "response_schema": content.Schema(
            type=content.Type.OBJECT,
            enum=[],
            required=["appendices"],
            properties={
                "appendices": content.Schema(
                    type=content.Type.ARRAY,
                    items=content.Schema(
                        type=content.Type.OBJECT,
                        enum=[],
                        required=["title", "description", "content"],
                        properties={
                            "title": content.Schema(
                                type=content.Type.STRING,
                            ),
                            "description": content.Schema(
                                type=content.Type.STRING,
                            ),
                            "content": content.Schema(
                                type=content.Type.STRING,
                            ),
                        },
                    ),
                ),
            },
        ),
        "response_mime_type": "application/json",
    }
    model_appendices = create_model(
        "gemini-2.0-flash-exp",
        generation_config_appendices,
        system_instruction="## **8. Appendices**\n\n- Include any additional information, data, or supplementary materials that support the project.\n- Label each appendix with a title or description for easy reference.\n- Number the appendices sequentially (Appendix A, Appendix B, etc.) for organization.\n",
    )
    try:
        response_appendices = model_appendices.generate_content(
            json.dumps(
                {
                    "projectInformation": project_info,
                    "projectContent": project_content,
                    "structureAndFormat": project_structure,
                    "additionalInstructions": project_instructions,
                    "abstract": project_abstract,
                    "projectDetails": project_details,
                    "conclusion": project_conclusion,
                    "references": project_references,
                }
            )
        )
        project_appendices = json.loads(response_appendices.text)
        return project_appendices
    except (json.JSONDecodeError, AttributeError) as e:
        logging.error(f"Error generating project appendices: {e}")
        return {}


def generate_full_report(
    project_info: Dict[str, Any],
    project_content: Dict[str, Any],
    project_structure: Dict[str, Any],
    project_instructions: Dict[str, Any],
    project_abstract: Dict[str, Any],
    project_details: Dict[str, Any],
    project_conclusion: Dict[str, Any],
    project_references: Dict[str, Any],
    project_appendices: Dict[str, Any],
) -> str:
    """Generates the full Markdown report string from the given data dictionaries.

    Args:
        project_info (Dict[str, Any]): The project information dictionary.
        project_content (Dict[str, Any]): The project content dictionary.
        project_structure (Dict[str, Any]): The project structure dictionary.
        project_instructions (Dict[str, Any]): The project instructions dictionary.
        project_abstract (Dict[str, Any]): The project abstract dictionary.
        project_details (Dict[str, Any]): The project details dictionary.
        project_conclusion (Dict[str, Any]): The project conclusion dictionary.
        project_references (Dict[str, Any]): The project references dictionary.
        project_appendices (Dict[str, Any]): The project appendices dictionary.

    Returns:
        str: A string containing the generated report.
    """
    template = Template(
        """
# Comprehensive Graduation Project Report
## Table of Contents
1. [Project Information](#project-information)
2. [Project Content](#project-content)
3. [Project Structure](#project-structure)
4. [Additional Instructions](#additional-instructions)
5. [Abstract](#abstract)
6. [Project Details](#project-details)
7. [Conclusion](#conclusion)
8. [References](#references)
9. [Appendices](#appendices)

## 1. Project Information
{{ project_info['projectInformation']['projectTitle'] }}
### 1.1 Project Scope
**Description:** {{ project_info['projectInformation']['projectScope']['description'] }}
**Objectives and Goals:**
{{ format_markdown_from_data(project_info['projectInformation']['projectScope']['objectivesAndGoals'], indent='  ', level = 0) }}
**Boundaries:** {{ project_info['projectInformation']['projectScope']['boundaries'] }}
**Stakeholder Identification:**
{{ format_markdown_from_data(project_info['projectInformation']['projectScope']['stakeholderIdentification'], indent='  ', level = 0) }}
**Timeline:**
- **Start:** {{ project_info['projectInformation']['projectScope']['timeline']['start'] }}
- **End:** {{ project_info['projectInformation']['projectScope']['timeline']['end'] }}
- **Milestones:** {{ format_markdown_from_data(project_info['projectInformation']['projectScope']['timeline']['milestones'], indent='  ', level = 0) }}
**Resource Allocation:**
{{ format_markdown_from_data(project_info['projectInformation']['projectScope']['resourceAllocation']['teamResponsibilities'], indent='  ', level = 0)}}
**Risk Assessment:** {{ project_info['projectInformation']['projectScope']['riskAssessment'] }}
**Approval Process:** {{ project_info['projectInformation']['projectScope']['approvalProcess'] }}
### 1.2 Field of Study
**Academic Field:** {{ project_info['projectInformation']['fieldOfStudy']['academicField'] }}
**Justification:**
{{ format_markdown_from_data(project_info['projectInformation']['fieldOfStudy']['justification'], indent='  ', level = 0) }}
### 1.3 Target Audience
**Market Research:** {{ project_info['projectInformation']['targetAudience']['marketResearch'] }}
**Demographics:**
{{ format_markdown_from_data(project_info['projectInformation']['targetAudience']['demographics'], indent='  ', level = 0) }}
**Psychographics:**
{{ format_markdown_from_data(project_info['projectInformation']['targetAudience']['psychographics'], indent='  ', level = 0) }}
**Audience Feedback:**
{{ format_markdown_from_data(project_info['projectInformation']['targetAudience']['audienceFeedback'], indent='  ', level = 0) }}
**Customer Data Analysis:** {{ project_info['projectInformation']['targetAudience']['customerDataAnalysis'] }}
**Audience Personas:**
{{ format_markdown_from_data(project_info['projectInformation']['targetAudience']['audiencePersonas'], indent='  ', level = 0) }}

## 2. Project Content
### 2.1 Problem Statement
**Description:** {{ project_content['projectContent']['problemStatement']['description'] }}
**Steps:**
{{ format_markdown_from_data(project_content['projectContent']['problemStatement']['steps'], indent='  ', level = 0) }}
### 2.2 Research Questions
**Description:** {{ project_content['projectContent']['researchQuestions']['description'] }}
**Steps:**
{{ format_markdown_from_data(project_content['projectContent']['researchQuestions']['steps'], indent='  ', level = 0) }}
### 2.3 Literature Review
**Description:** {{ project_content['projectContent']['literatureReview']['description'] }}
**Steps:**
{{ format_markdown_from_data(project_content['projectContent']['literatureReview']['steps'], indent='  ', level = 0) }}
### 2.4 Methodology
**Description:** {{ project_content['projectContent']['methodology']['description'] }}
**Steps:**
{{ format_markdown_from_data(project_content['projectContent']['methodology']['steps'], indent='  ', level = 0) }}
### 2.5 System Design
**Description:** {{ project_content['projectContent']['systemDesign']['description'] }}
**Steps:**
{{ format_markdown_from_data(project_content['projectContent']['systemDesign']['steps'], indent='  ', level = 0) }}
### 2.6 Implementation
**Description:** {{ project_content['projectContent']['implementation']['description'] }}
**Steps:**
{{ format_markdown_from_data(project_content['projectContent']['implementation']['steps'], indent='  ', level = 0) }}
### 2.7 Testing and Validation
**Description:** {{ project_content['projectContent']['testingAndValidation']['description'] }}
**Steps:**
{{ format_markdown_from_data(project_content['projectContent']['testingAndValidation']['steps'], indent='  ', level = 0) }}
### 2.8 Results and Analysis
**Description:** {{ project_content['projectContent']['resultsAndAnalysis']['description'] }}
**Steps:**
{{ format_markdown_from_data(project_content['projectContent']['resultsAndAnalysis']['steps'], indent='  ', level = 0) }}
### 2.9 Future Work
**Description:** {{ project_content['projectContent']['futureWork']['description'] }}
**Steps:**
{{ format_markdown_from_data(project_content['projectContent']['futureWork']['steps'], indent='  ', level = 0) }}
### 2.10 Contributions
**Description:** {{ project_content['projectContent']['contributions']['description'] }}
**Steps:**
{{ format_markdown_from_data(project_content['projectContent']['contributions']['steps'], indent='  ', level = 0) }}

## 3. Project Structure
### 3.1 Target Length
{{ project_structure['structureAndFormat']['targetLength'] }}
### 3.2 Chapter Structure
**Chapters:**
{{ format_markdown_from_data(project_structure['structureAndFormat']['chapterStructure']['chapters'], indent='  ', level = 0) }}
**Description:** {{ project_structure['structureAndFormat']['chapterStructure']['description'] }}
### 3.3 Visual Aids
**Diagrams:**
{{ format_markdown_from_data(project_structure['structureAndFormat']['visualAids']['diagram'], indent='  ', level = 0) }}

## 4. Additional Instructions
### 4.1 Tone and Style
{{ format_markdown_from_data(project_instructions['additionalInstructions']['toneAndStyle'], indent='  ', level = 0) }}
### 4.2 Keywords
{{ format_markdown_from_data(project_instructions['additionalInstructions']['keywords'], indent='  ', level = 0) }}
### 4.3 Constraints
{{ project_instructions['additionalInstructions']['constraints'] }}

## 5. Abstract
{{ project_abstract['abstract'] }}

## 6. Project Details
### 6.1 Overview
{{ project_details['projectDetails']['overview'] }}
### 6.2 Project Planning
**Define Scope:** {{ project_details['projectDetails']['projectPlanning']['defineScope'] }}
**Establish Timeline:** {{ project_details['projectDetails']['projectPlanning']['establishTimeline'] }}
**Identify Stakeholders:** {{ project_details['projectDetails']['projectPlanning']['identifyStakeholders'] }}
**Allocate Resources:** {{ project_details['projectDetails']['projectPlanning']['allocateResources'] }}
### 6.3 Requirements Gathering
**Conduct Interviews:** {{ project_details['projectDetails']['requirementsGathering']['conductInterviews'] }}
**Review Documentation:** {{ project_details['projectDetails']['requirementsGathering']['reviewDocumentation'] }}
**Compile User Requirements:** {{ project_details['projectDetails']['requirementsGathering']['compileUserRequirements'] }}
### 6.4 Hardware Requirements
**Computing Power:**
- **CPU:** {{ project_details['projectDetails']['hardwareRequirements']['computingPower']['cpu'] }}
- **RAM:** {{ project_details['projectDetails']['hardwareRequirements']['computingPower']['ram'] }}
- **Storage:** {{ project_details['projectDetails']['hardwareRequirements']['computingPower']['storage'] }}
**Peripheral Devices:**
- **Input Devices:** {{ project_details['projectDetails']['hardwareRequirements']['peripheralDevices']['inputDevices'] }}
- **Output Devices:** {{ project_details['projectDetails']['hardwareRequirements']['peripheralDevices']['outputDevices'] }}
**Network Connectivity:** {{ project_details['projectDetails']['hardwareRequirements']['networkConnectivity'] }}
**Scalability:** {{ project_details['projectDetails']['hardwareRequirements']['scalability'] }}
### 6.5 Software Requirements
**Operating System:** {{ project_details['projectDetails']['softwareRequirements']['operatingSystem'] }}
**Software Applications:**
- **List of Applications:** {{ project_details['projectDetails']['softwareRequirements']['softwareApplications']['listApplications'] }}
- **Version Compatibility:** {{ project_details['projectDetails']['softwareRequirements']['softwareApplications']['versionCompatibility'] }}
**Database Requirements:** {{ project_details['projectDetails']['softwareRequirements']['databaseRequirements'] }}
**Security Software Needs:** {{ project_details['projectDetails']['softwareRequirements']['securitySoftwareNeeds'] }}
### 6.6 Design Phase
**System Architecture Diagrams:** {{ project_details['projectDetails']['designPhase']['systemArchitectureDiagrams'] }}
**User Interface Mockups:** {{ project_details['projectDetails']['designPhase']['userInterfaceMockups'] }}
**Database Schema:** {{ project_details['projectDetails']['designPhase']['databaseSchema'] }}
### 6.7 Implementation Phase
**Hardware Setup:** {{ project_details['projectDetails']['implementationPhase']['hardwareSetup'] }}
**Software Installation:**
- **Install OS:** {{ project_details['projectDetails']['implementationPhase']['softwareInstallation']['installOS'] }}
- **Install Applications:** {{ project_details['projectDetails']['implementationPhase']['softwareInstallation']['installApplications'] }}
**Code Development:** {{ project_details['projectDetails']['implementationPhase']['codeDevelopment'] }}
### 6.8 Testing Phase
**Create Test Plan:** {{ project_details['projectDetails']['testingPhase']['createTestPlan'] }}
**Unit Testing:** {{ project_details['projectDetails']['testingPhase']['unitTesting'] }}
**Integration Testing:** {{ project_details['projectDetails']['testingPhase']['integrationTesting'] }}
**User Acceptance Testing:** {{ project_details['projectDetails']['testingPhase']['userAcceptanceTesting'] }}
### 6.9 Deployment Phase
**Deployment Plan:** {{ project_details['projectDetails']['deploymentPhase']['deploymentPlan'] }}
**User Training:** {{ project_details['projectDetails']['deploymentPhase']['userTraining'] }}
### 6.10 Maintenance and Support
**Maintenance Plan:** {{ project_details['projectDetails']['maintenanceAndSupport']['maintenancePlan'] }}
**Support Services:** {{ project_details['projectDetails']['maintenanceAndSupport']['supportServices'] }}
### 6.11 Documentation
**User Manual:** {{ project_details['projectDetails']['documentation']['userManual'] }}
**Technical Documentation:** {{ project_details['projectDetails']['documentation']['technicalDocumentation'] }}

## 7. Conclusion
### 7.1 Summary of Findings
{{ project_conclusion['conclusion']['summaryOfFindings'] }}
### 7.2 Achievements
{{ project_conclusion['conclusion']['achievements'] }}
### 7.3 Lessons Learned
{{ project_conclusion['conclusion']['lessonsLearned'] }}
### 7.4 Future Directions
{{ project_conclusion['conclusion']['futureDirections'] }}
### 7.5 Final Thoughts
{{ project_conclusion['conclusion']['finalThoughts'] }}



## 8. References
{{ format_markdown_from_data(project_references['references'], indent='  ', level = 0) }}


## 9. Appendices
{{ format_markdown_from_data(project_appendices['appendices'], indent='  ', level = 0) }}
"""
    )

    return template.render(
        project_info=project_info,
        project_content=project_content,
        project_structure=project_structure,
        project_instructions=project_instructions,
        project_abstract=project_abstract,
        project_details=project_details,
        project_conclusion=project_conclusion,
        project_references=project_references,
        project_appendices=project_appendices,
        format_markdown_from_data=format_markdown_from_data,
    )


def main():
    """Main function to orchestrate the generation of the graduation project report."""
    initial_project_title = "Automated System for Generating Comprehensive Graduation Project Reports Using PlantUML"
    logging.info("Generating project titles...")

    titles = generate_project_titles(initial_project_title)
    if not titles:
        logging.error("Failed to generate project titles. Exiting.")
        return

    selected_title = get_selected_title(titles)
    if not selected_title:
        logging.error("No title selected. Exiting.")
        return

    project_info = generate_project_info(selected_title)
    if not project_info:
        logging.error("Failed to generate project information. Exiting.")
        return

    project_content = generate_project_content(project_info)
    if not project_content:
        logging.error("Failed to generate project content. Exiting.")
        return

    project_structure = generate_project_structure(project_info)
    if not project_structure:
        logging.error("Failed to generate project structure. Exiting.")
        return

    project_instructions = generate_additional_instructions(
        project_info, project_content, project_structure
    )
    if not project_instructions:
        logging.error("Failed to generate project instructions. Exiting.")
        return

    project_abstract = generate_abstract(
        project_info, project_content, project_structure, project_instructions
    )
    if not project_abstract:
        logging.error("Failed to generate project abstract. Exiting.")
        return

    project_details = generate_project_details(
        project_info,
        project_content,
        project_structure,
        project_instructions,
        project_abstract,
    )
    if not project_details:
        logging.error("Failed to generate project details. Exiting.")
        return

    project_conclusion = generate_conclusion(
        project_info,
        project_content,
        project_structure,
        project_instructions,
        project_abstract,
        project_details,
    )
    if not project_conclusion:
        logging.error("Failed to generate project conclusion. Exiting.")
        return

    project_references = generate_references(
        project_info,
        project_content,
        project_structure,
        project_instructions,
        project_abstract,
        project_details,
        project_conclusion,
    )
    if not project_references:
        logging.error("Failed to generate project references. Exiting.")
        return

    project_appendices = generate_appendices(
        project_info,
        project_content,
        project_structure,
        project_instructions,
        project_abstract,
        project_details,
        project_conclusion,
        project_references,
    )
    if not project_appendices:
        logging.error("Failed to generate project appendices. Exiting.")
        return

    full_report = generate_full_report(
        project_info,
        project_content,
        project_structure,
        project_instructions,
        project_abstract,
        project_details,
        project_conclusion,
        project_references,
        project_appendices,
    )
    # save full_report.md full_report
    with open("full_report.md", "w") as f:
        f.write(full_report)
    # display full_report
    display(Markdown(full_report))

    logging.info("Project report generation completed.")


if __name__ == "__main__":
    main()
