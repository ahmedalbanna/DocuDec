import google.generativeai as genai
from google.ai.generativelanguage_v1beta.types import content

Project_Instructions = "**Graduation Project Report Prompt Template**  **1. Project Information:**  * **Project Title:** [Enter the project title] * **Project Scope:** [Briefly describe the project's scope and objectives] * **Field of Study:** [Specify the academic field of the project] * **Target Audience:** [Who is the intended audience for this report?]  **2. Content:**  * **Problem Statement:** [Clearly define the problem the project aims to solve, including its context and significance] * **Research Questions:** [List any specific research questions addressed by the project] * **Literature Review:** [Summarize the key findings from your research on existing solutions, technologies, or related work] * **Methodology:**  [Describe the approach and methodology used to design, implement, and test the project] * **System Design:** [Explain the project's architecture, including its components, data structures, and algorithms] * **Implementation:** [Detail the programming languages and techniques used to develop the project, including any libraries or frameworks] * **Testing and Validation:** [Describe the testing process, including test cases, results, and any issues encountered] * **Results and Analysis:** [Present the key findings and results of the project, analyzing their implications] * **Future Work:** [Discuss potential future enhancements, improvements, or applications of the project] * **Contributions:** [Highlight the project's contributions to your field of study]  **3.  Structure and Format:**  * **Target Length:** [Specify the desired length of the report (e.g., 10-15 pages)] * **Chapter Structure:** [Outline the desired structure of the report, including chapter headings and sub-sections] * **Visual Aids:** [Specify any desired visual aids, such as diagrams, charts, or tables]  **4. Additional Instructions:**  * **Tone and Style:** [Indicate the desired tone and style of the report, such as formal, informal, academic, etc.] * **Keywords:**  [Include any specific keywords or phrases you want the generator to use] * **Constraints:**  [Specify any specific constraints, such as word count limitations, specific formats, or required elements]"

# Create the model
generation_config = {
    "temperature": 1,
    "top_p": 0.95,
    "top_k": 40,
    "max_output_tokens": 852,
    "response_schema": content.Schema(
        type = content.Type.OBJECT,
        enum = [],
        required = ["project"],
        properties = {
            "project": content.Schema(
                type = content.Type.OBJECT,
                enum = [],
                required = ["title", "description", "objectives", "features", "testingValidation", "benefits", "challenges", "targetAudience", "development", "futureEnhancements", "stakeholders"],
                properties = {
                    "title": content.Schema(
                        type = content.Type.STRING,
                    ),
                    "description": content.Schema(
                        type = content.Type.STRING,
                    ),
                    "objectives": content.Schema(
                        type = content.Type.ARRAY,
                        items = content.Schema(
                            type = content.Type.STRING,
                        ),
                    ),
                    "features": content.Schema(
                        type = content.Type.OBJECT,
                        enum = [],
                        required = ["contentGeneration", "diagrammingTools", "userInterface"],
                        properties = {
                            "contentGeneration": content.Schema(
                                type = content.Type.STRING,
                            ),
                            "diagrammingTools": content.Schema(
                                type = content.Type.STRING,
                            ),
                            "userInterface": content.Schema(
                                type = content.Type.STRING,
                            ),
                        },
                    ),
                    "testingValidation": content.Schema(
                        type = content.Type.STRING,
                    ),
                    "benefits": content.Schema(
                        type = content.Type.ARRAY,
                        items = content.Schema(
                            type = content.Type.STRING,
                        ),
                    ),
                    "challenges": content.Schema(
                        type = content.Type.ARRAY,
                        items = content.Schema(
                            type = content.Type.STRING,
                        ),
                    ),
                    "targetAudience": content.Schema(
                        type = content.Type.ARRAY,
                        items = content.Schema(
                            type = content.Type.STRING,
                        ),
                    ),
                    "development": content.Schema(
                        type = content.Type.OBJECT,
                        enum = [],
                        required = ["methodology", "technologies", "security"],
                        properties = {
                            "methodology": content.Schema(
                                type = content.Type.STRING,
                            ),
                            "technologies": content.Schema(
                                type = content.Type.ARRAY,
                                items = content.Schema(
                                    type = content.Type.STRING,
                                ),
                            ),
                            "security": content.Schema(
                                type = content.Type.STRING,
                            ),
                        },
                    ),
                    "futureEnhancements": content.Schema(
                        type = content.Type.ARRAY,
                        items = content.Schema(
                            type = content.Type.STRING,
                        ),
                    ),
                    "stakeholders": content.Schema(
                        type = content.Type.ARRAY,
                        items = content.Schema(
                            type = content.Type.STRING,
                        ),
                    ),
                },
            ),
        },
    ),
    "response_mime_type": "application/json",
}

model = genai.GenerativeModel(
    model_name="gemini-2.0-flash-exp",
    generation_config=generation_config,
    system_instruction = Project_Instructions,
    safety_settings=[
            {"category": "HARM_CATEGORY_HARASSMENT", "threshold": "block_none"},
            {"category": "HARM_CATEGORY_DANGEROUS_CONTENT", "threshold": "block_none"},
            {"category": "HARM_CATEGORY_HATE_SPEECH", "threshold": "block_none"},
        ],
)

prompt = "Create a compelling and descriptive project title based on the project scope, field of study, and target audience. Ensure the title reflects the core objectives and significance of the project."

def generate_project(project_title):
    response = model.generate_content([
    f"project: {project_title}",
    f"prompt: {prompt}",
    ])
    return response.text