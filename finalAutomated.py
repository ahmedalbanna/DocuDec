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

cache_dir = ".cache"  # Path to your cache directory
TEMPERATURE_HIGH = 1.5
TEMPERATURE_NORMAL = 1
TOP_P = 0.95
TOP_K = 40
MAX_OUTPUT_TOKENS_TITLES = 500
MAX_OUTPUT_TOKENS_PROJECT_INFO = 1500
MAX_OUTPUT_TOKENS_PROJECT_CONTENT = 2500
MAX_OUTPUT_TOKENS_STRUCTURE = 1500
MAX_OUTPUT_TOKENS_INSTRUCTIONS = 2891
MAX_OUTPUT_TOKENS_ABSTRACT = 8192
MAX_OUTPUT_TOKENS_PROJECT_DETAILS = 8192
MAX_OUTPUT_TOKENS_CONCLUSION = 8192
MAX_OUTPUT_TOKENS_REFERENCES = 8192
MAX_OUTPUT_TOKENS_APPENDICES = 8192
CACHE_EXPIRY_SECONDS = 3600  # 1 hour

# --- Logging setup ---
logger = logging.getLogger(__name__)
logger.setLevel(logging.INFO)
formatter = logging.Formatter("%(asctime)s - %(levelname)s - %(message)s")
ch = logging.StreamHandler()
ch.setFormatter(formatter)
logger.addHandler(ch)
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
        "temperature": TEMPERATURE_HIGH,
        "top_p": TOP_P,
        "top_k": TOP_K,
        "max_output_tokens": MAX_OUTPUT_TOKENS_TITLES,
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
        "temperature": TEMPERATURE_NORMAL,
        "top_p": TOP_P,
        "top_k": TOP_K,
        "max_output_tokens": MAX_OUTPUT_TOKENS_PROJECT_INFO,
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
        "temperature": TEMPERATURE_NORMAL,
        "top_p": TOP_P,
        "top_k": TOP_K,
        "max_output_tokens": MAX_OUTPUT_TOKENS_PROJECT_CONTENT,
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
        "temperature": TEMPERATURE_NORMAL,
        "top_p": TOP_P,
        "top_k": TOP_K,
        "max_output_tokens": MAX_OUTPUT_TOKENS_STRUCTURE,
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
        "temperature": TEMPERATURE_NORMAL,
        "top_p": TOP_P,
        "top_k": TOP_K,
        "max_output_tokens": MAX_OUTPUT_TOKENS_INSTRUCTIONS,
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
        "temperature": TEMPERATURE_HIGH,
        "top_p": TOP_P,
        "top_k": TOP_K,
        "max_output_tokens": MAX_OUTPUT_TOKENS_ABSTRACT,
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
        "temperature": TEMPERATURE_NORMAL,
        "top_p": TOP_P,
        "top_k": TOP_K,
        "max_output_tokens": MAX_OUTPUT_TOKENS_PROJECT_DETAILS,
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
        "temperature": TEMPERATURE_NORMAL,
        "top_p": TOP_P,
        "top_k": TOP_K,
        "max_output_tokens": MAX_OUTPUT_TOKENS_CONCLUSION,
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
        "temperature": TEMPERATURE_NORMAL,
        "top_p": TOP_P,
        "top_k": TOP_K,
        "max_output_tokens": MAX_OUTPUT_TOKENS_REFERENCES,
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
        "temperature": TEMPERATURE_NORMAL,
        "top_p": TOP_P,
        "top_k": TOP_K,
        "max_output_tokens": MAX_OUTPUT_TOKENS_APPENDICES,
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


def generate_table_of_contents() -> str:
    """Generates the table of contents for the report."""
    return """
## Table of Contents
1.  [Project Information](#project-information)
2.  [Project Content](#project-content)
3.  [Project Structure](#project-structure)
4.  [Additional Instructions](#additional-instructions)
5.  [Abstract](#abstract)
6.  [Project Details](#project-details)
7.  [Conclusion](#conclusion)
8.  [References](#references)
9.  [Appendices](#appendices)
"""


def generate_project_information_section(project_info: Dict[str, Any]) -> str:
    """Generates the project information section of the report."""
    if not project_info or "projectInformation" not in project_info:
        return "## 1. Project Information\n\nData not available."

    pi = project_info["projectInformation"]
    project_title = pi.get("projectTitle", "N/A")
    scope_description = pi.get("projectScope", {}).get("description", "N/A")
    scope_objectives = pi.get("projectScope", {}).get("objectivesAndGoals", [])
    scope_boundaries = pi.get("projectScope", {}).get("boundaries", "N/A")
    scope_stakeholders = pi.get("projectScope", {}).get("stakeholderIdentification", [])
    scope_timeline_start = (
        pi.get("projectScope", {}).get("timeline", {}).get("start", "N/A")
    )
    scope_timeline_end = (
        pi.get("projectScope", {}).get("timeline", {}).get("end", "N/A")
    )
    scope_timeline_milestones = (
        pi.get("projectScope", {}).get("timeline", {}).get("milestones", [])
    )
    scope_resource_allocation = (
        pi.get("projectScope", {})
        .get("resourceAllocation", {})
        .get("teamResponsibilities", [])
    )
    scope_risk_assessment = pi.get("projectScope", {}).get("riskAssessment", "N/A")
    scope_approval_process = pi.get("projectScope", {}).get("approvalProcess", "N/A")
    field_academic = pi.get("fieldOfStudy", {}).get("academicField", "N/A")
    field_justification = pi.get("fieldOfStudy", {}).get("justification", [])
    target_market_research = pi.get("targetAudience", {}).get("marketResearch", "N/A")
    target_demographics = pi.get("targetAudience", {}).get("demographics", {})
    target_psychographics = pi.get("targetAudience", {}).get("psychographics", [])
    target_audience_feedback = pi.get("targetAudience", {}).get("audienceFeedback", [])
    target_customer_data = pi.get("targetAudience", {}).get(
        "customerDataAnalysis", "N/A"
    )
    target_audience_personas = pi.get("targetAudience", {}).get("audiencePersonas", [])

    return f"""
## 1. Project Information
{project_title}
### 1.1 Project Scope
**Description:** {scope_description}\n
**Objectives and Goals:**
{format_markdown_from_data(scope_objectives, indent='  ', level=0)}
**Boundaries:** {scope_boundaries}\n
**Stakeholder Identification:**
{format_markdown_from_data(scope_stakeholders, indent='  ', level=0)}
**Timeline:**
- **Start:** {scope_timeline_start}
- **End:** {scope_timeline_end}
- **Milestones:** {format_markdown_from_data(scope_timeline_milestones, indent='  ', level=0)}
**Resource Allocation:**
{format_markdown_from_data(scope_resource_allocation, indent='  ', level=0)}
**Risk Assessment:** {scope_risk_assessment}
**Approval Process:** {scope_approval_process}
### 1.2 Field of Study
**Academic Field:** {field_academic}\n
**Justification:**
{format_markdown_from_data(field_justification, indent='  ', level=0)}
### 1.3 Target Audience
**Market Research:** {target_market_research}\n
**Demographics:**
{format_markdown_from_data(target_demographics, indent='  ', level=0)}
**Psychographics:**
{format_markdown_from_data(target_psychographics, indent='  ', level=0)}
**Audience Feedback:**
{format_markdown_from_data(target_audience_feedback, indent='  ', level=0)}
**Customer Data Analysis:** {target_customer_data}\n
**Audience Personas:**
{format_markdown_from_data(target_audience_personas, indent='  ', level=0)}
"""


def generate_project_content_section(project_content: Dict[str, Any]) -> str:
    """Generates the project content section of the report."""
    if not project_content or "projectContent" not in project_content:
        return "## 2. Project Content\n\nData not available."

    pc = project_content["projectContent"]
    problem_description = pc.get("problemStatement", {}).get("description", "N/A")
    problem_steps = pc.get("problemStatement", {}).get("steps", [])
    research_description = pc.get("researchQuestions", {}).get("description", "N/A")
    research_steps = pc.get("researchQuestions", {}).get("steps", [])
    literature_description = pc.get("literatureReview", {}).get("description", "N/A")
    literature_steps = pc.get("literatureReview", {}).get("steps", [])
    methodology_description = pc.get("methodology", {}).get("description", "N/A")
    methodology_steps = pc.get("methodology", {}).get("steps", [])
    system_description = pc.get("systemDesign", {}).get("description", "N/A")
    system_steps = pc.get("systemDesign", {}).get("steps", [])
    implementation_description = pc.get("implementation", {}).get("description", "N/A")
    implementation_steps = pc.get("implementation", {}).get("steps", [])
    testing_description = pc.get("testingAndValidation", {}).get("description", "N/A")
    testing_steps = pc.get("testingAndValidation", {}).get("steps", [])
    results_description = pc.get("resultsAndAnalysis", {}).get("description", "N/A")
    results_steps = pc.get("resultsAndAnalysis", {}).get("steps", [])
    future_description = pc.get("futureWork", {}).get("description", "N/A")
    future_steps = pc.get("futureWork", {}).get("steps", [])
    contributions_description = pc.get("contributions", {}).get("description", "N/A")
    contributions_steps = pc.get("contributions", {}).get("steps", [])

    return f"""
## 2. Project Content
### 2.1 Problem Statement
**Description:** {problem_description}\n
**Steps:**
{format_markdown_from_data(problem_steps, indent='  ', level=0)}
### 2.2 Research Questions
**Description:** {research_description}\n
**Steps:**
{format_markdown_from_data(research_steps, indent='  ', level=0)}
### 2.3 Literature Review
**Description:** {literature_description}\n
**Steps:**
{format_markdown_from_data(literature_steps, indent='  ', level=0)}
### 2.4 Methodology
**Description:** {methodology_description}\n
**Steps:**
{format_markdown_from_data(methodology_steps, indent='  ', level=0)}
### 2.5 System Design
**Description:** {system_description}\n
**Steps:**
{format_markdown_from_data(system_steps, indent='  ', level=0)}
### 2.6 Implementation
**Description:** {implementation_description}\n
**Steps:**
{format_markdown_from_data(implementation_steps, indent='  ', level=0)}
### 2.7 Testing and Validation
**Description:** {testing_description}\n
**Steps:**
{format_markdown_from_data(testing_steps, indent='  ', level=0)}
### 2.8 Results and Analysis
**Description:** {results_description}\n
**Steps:**
{format_markdown_from_data(results_steps, indent='  ', level=0)}
### 2.9 Future Work
**Description:** {future_description}\n
**Steps:**
{format_markdown_from_data(future_steps, indent='  ', level=0)}
### 2.10 Contributions
**Description:** {contributions_description}\n
**Steps:**
{format_markdown_from_data(contributions_steps, indent='  ', level=0)}
"""


def generate_project_structure_section(project_structure: Dict[str, Any]) -> str:
    """Generates the project structure section of the report."""
    if not project_structure or "structureAndFormat" not in project_structure:
        return "## 3. Project Structure\n\nData not available."

    ps = project_structure["structureAndFormat"]
    target_length = ps.get("targetLength", "N/A")
    chapter_chapters = ps.get("chapterStructure", {}).get("chapters", [])
    chapter_description = ps.get("chapterStructure", {}).get("description", "N/A")
    visual_diagrams = ps.get("visualAids", {}).get("diagram", [])

    return f"""
## 3. Project Structure
### 3.1 Target Length
{target_length}
### 3.2 Chapter Structure
**Chapters:**
{format_markdown_from_data(chapter_chapters, indent='  ', level=0)}
**Description:** {chapter_description}
### 3.3 Visual Aids
**Diagrams:**
{format_markdown_from_data(visual_diagrams, indent='  ', level=0)}
"""


def generate_additional_instructions_section(
    project_instructions: Dict[str, Any]
) -> str:
    """Generates the additional instructions section of the report."""
    if not project_instructions or "additionalInstructions" not in project_instructions:
        return "## 4. Additional Instructions\n\nData not available."

    ai = project_instructions["additionalInstructions"]
    tone_and_style = ai.get("toneAndStyle", [])
    keywords = ai.get("keywords", [])
    constraints = ai.get("constraints", "N/A")
    return f"""
## 4. Additional Instructions
### 4.1 Tone and Style
{format_markdown_from_data(tone_and_style, indent='  ', level=0)}
### 4.2 Keywords
{format_markdown_from_data(keywords, indent='  ', level=0)}
### 4.3 Constraints
{constraints}
"""


def generate_abstract_section(project_abstract: Dict[str, Any]) -> str:
    """Generates the abstract section of the report."""
    if not project_abstract or "abstract" not in project_abstract:
        return "## 5. Abstract\n\nData not available."
    abstract = project_abstract["abstract"]

    return f"""
## 5. Abstract
{abstract}
"""


def generate_project_details_section(project_details: Dict[str, Any]) -> str:
    """Generates the project details section of the report."""
    if not project_details or "projectDetails" not in project_details:
        return "## 6. Project Details\n\nData not available."

    pd = project_details["projectDetails"]
    overview = pd.get("overview", "N/A")
    planning_define = pd.get("projectPlanning", {}).get("defineScope", "N/A")
    planning_timeline = pd.get("projectPlanning", {}).get("establishTimeline", "N/A")
    planning_stakeholders = pd.get("projectPlanning", {}).get(
        "identifyStakeholders", "N/A"
    )
    planning_resources = pd.get("projectPlanning", {}).get("allocateResources", "N/A")
    requirements_interviews = pd.get("requirementsGathering", {}).get(
        "conductInterviews", "N/A"
    )
    requirements_documentation = pd.get("requirementsGathering", {}).get(
        "reviewDocumentation", "N/A"
    )
    requirements_compile = pd.get("requirementsGathering", {}).get(
        "compileUserRequirements", "N/A"
    )
    hardware_cpu = (
        pd.get("hardwareRequirements", {}).get("computingPower", {}).get("cpu", "N/A")
    )
    hardware_ram = (
        pd.get("hardwareRequirements", {}).get("computingPower", {}).get("ram", "N/A")
    )
    hardware_storage = (
        pd.get("hardwareRequirements", {})
        .get("computingPower", {})
        .get("storage", "N/A")
    )
    hardware_input = (
        pd.get("hardwareRequirements", {})
        .get("peripheralDevices", {})
        .get("inputDevices", "N/A")
    )
    hardware_output = (
        pd.get("hardwareRequirements", {})
        .get("peripheralDevices", {})
        .get("outputDevices", "N/A")
    )
    hardware_network = pd.get("hardwareRequirements", {}).get(
        "networkConnectivity", "N/A"
    )
    hardware_scalability = pd.get("hardwareRequirements", {}).get("scalability", "N/A")
    software_os = pd.get("softwareRequirements", {}).get("operatingSystem", "N/A")
    software_applications = pd.get("softwareRequirements", {}).get(
        "softwareApplications", {}
    )
    software_applications_list = software_applications.get("listApplications", "N/A")
    software_applications_version = software_applications.get(
        "versionCompatibility", "N/A"
    )
    software_database = pd.get("softwareRequirements", {}).get(
        "databaseRequirements", "N/A"
    )
    software_security = pd.get("softwareRequirements", {}).get(
        "securitySoftwareNeeds", "N/A"
    )
    design_diagrams = pd.get("designPhase", {}).get("systemArchitectureDiagrams", "N/A")
    design_mockups = pd.get("designPhase", {}).get("userInterfaceMockups", "N/A")
    design_schema = pd.get("designPhase", {}).get("databaseSchema", "N/A")
    implementation_setup = pd.get("implementationPhase", {}).get("hardwareSetup", "N/A")
    implementation_software = pd.get("implementationPhase", {}).get(
        "softwareInstallation", {}
    )
    implementation_os = implementation_software.get("installOS", "N/A")
    implementation_applications = implementation_software.get(
        "installApplications", "N/A"
    )
    implementation_code = pd.get("implementationPhase", {}).get(
        "codeDevelopment", "N/A"
    )
    testing_plan = pd.get("testingPhase", {}).get("createTestPlan", "N/A")
    testing_unit = pd.get("testingPhase", {}).get("unitTesting", "N/A")
    testing_integration = pd.get("testingPhase", {}).get("integrationTesting", "N/A")
    testing_acceptance = pd.get("testingPhase", {}).get("userAcceptanceTesting", "N/A")
    deployment_plan = pd.get("deploymentPhase", {}).get("deploymentPlan", "N/A")
    deployment_training = pd.get("deploymentPhase", {}).get("userTraining", "N/A")
    maintenance_plan = pd.get("maintenanceAndSupport", {}).get("maintenancePlan", "N/A")
    maintenance_services = pd.get("maintenanceAndSupport", {}).get(
        "supportServices", "N/A"
    )
    documentation_manual = pd.get("documentation", {}).get("userManual", "N/A")
    documentation_technical = pd.get("documentation", {}).get(
        "technicalDocumentation", "N/A"
    )

    return f"""
## 6. Project Details
### 6.1 Overview
{overview}
### 6.2 Project Planning\n
**Define Scope:** {planning_define}\n
**Establish Timeline:** {planning_timeline}\n
**Identify Stakeholders:** {planning_stakeholders}\n
**Allocate Resources:** {planning_resources}\n
### 6.3 Requirements Gathering\n
**Conduct Interviews:** {requirements_interviews}\n
**Review Documentation:** {requirements_documentation}\n
**Compile User Requirements:** {requirements_compile}\n
### 6.4 Hardware Requirements\n
**Computing Power:**\n
- **CPU:** {hardware_cpu}\n
- **RAM:** {hardware_ram}\n
- **Storage:** {hardware_storage}\n
**Peripheral Devices:**\n
- **Input Devices:** {hardware_input}\n
- **Output Devices:** {hardware_output}\n
**Network Connectivity:** {hardware_network}\n
**Scalability:** {hardware_scalability}\n
### 6.5 Software Requirements\n
**Operating System:** {software_os}\n
**Software Applications:**\n
- **List of Applications:** {software_applications_list}\n
- **Version Compatibility:** {software_applications_version}\n
**Database Requirements:** {software_database}\n
**Security Software Needs:** {software_security}\n
### 6.6 Design Phase\n
**System Architecture Diagrams:** {design_diagrams}\n
**User Interface Mockups:** {design_mockups}\n
**Database Schema:** {design_schema}\n
### 6.7 Implementation Phase\n
**Hardware Setup:** {implementation_setup}\n
**Software Installation:**\n
- **Install OS:** {implementation_os}\n
- **Install Applications:** {implementation_applications}\n
**Code Development:** {implementation_code}\n
### 6.8 Testing Phase\n
**Create Test Plan:** {testing_plan}\n
**Unit Testing:** {testing_unit}\n
**Integration Testing:** {testing_integration}\n
**User Acceptance Testing:** {testing_acceptance}\n
### 6.9 Deployment Phase\n
**Deployment Plan:** {deployment_plan}\n
**User Training:** {deployment_training}\n
### 6.10 Maintenance and Support\n
**Maintenance Plan:** {maintenance_plan}\n
**Support Services:** {maintenance_services}\n
### 6.11 Documentation\n
**User Manual:** {documentation_manual}\n
**Technical Documentation:** {documentation_technical}\n
"""


def generate_conclusion_section(project_conclusion: Dict[str, Any]) -> str:
    """Generates the conclusion section of the report."""
    if not project_conclusion or "conclusion" not in project_conclusion:
        return "## 7. Conclusion\n\nData not available."

    pc = project_conclusion["conclusion"]
    summary = pc.get("summaryOfFindings", "N/A")
    achievements = pc.get("achievements", "N/A")
    lessons = pc.get("lessonsLearned", "N/A")
    future = pc.get("futureDirections", "N/A")
    final = pc.get("finalThoughts", "N/A")

    return f"""
## 7. Conclusion
### 7.1 Summary of Findings
{summary}
### 7.2 Achievements
{achievements}
### 7.3 Lessons Learned
{lessons}
### 7.4 Future Directions
{future}
### 7.5 Final Thoughts
{final}
"""


def generate_references_section(project_references: Dict[str, Any]) -> str:
    """Generates the references section of the report."""
    if not project_references or "references" not in project_references:
        return "## 8. References\n\nData not available."
    references = project_references["references"]

    return f"""
## 8. References
{format_markdown_from_data(references, indent='  ', level=0)}
"""


def generate_appendices_section(project_appendices: Dict[str, Any]) -> str:
    """Generates the appendices section of the report."""
    if not project_appendices or "appendices" not in project_appendices:
        return "## 9. Appendices\n\nData not available."
    appendices = project_appendices["appendices"]

    return f"""
## 9. Appendices
{format_markdown_from_data(appendices, indent='  ', level=0)}
"""


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
    """Generates the full Markdown report string from the given data dictionaries."""

    report_parts = [
        "# Comprehensive Graduation Project Report",
        generate_table_of_contents(),
        generate_project_information_section(project_info),
        generate_project_content_section(project_content),
        generate_project_structure_section(project_structure),
        generate_additional_instructions_section(project_instructions),
        generate_abstract_section(project_abstract),
        generate_project_details_section(project_details),
        generate_conclusion_section(project_conclusion),
        generate_references_section(project_references),
        generate_appendices_section(project_appendices),
    ]
    return "\n".join(report_parts)


# save the markdown report to a file
def save_markdown_report(report: str, filename: str = "report.md") -> None:
    """Saves the generated Markdown report to a file."""
    with open(filename, "w") as file:
        file.write(report)
    return filename


# Display the markdown report
def display_markdown_report(report: str) -> None:
    """Displays the generated Markdown report."""
    display(Markdown(report))


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
