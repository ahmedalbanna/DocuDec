import google.generativeai as genai
from google.ai.generativelanguage_v1beta.types import content
from pydantic import BaseModel


# from ..configure_gemeni import configureGemeni
# project_generator, report_generator = configureGemeni()
# Graduation Project Report Prompt Template
# 1. Project Information:
# Project Title: [Enter the project title]
# Project Scope: [Briefly describe the project's scope and objectives]
# Field of Study: [Specify the academic field of the project]
# Target Audience: [Who is the intended audience for this report?]
# 2. Content:
# Problem Statement: [Clearly define the problem the project aims to solve, including its context and significance]
# Research Questions: [List any specific research questions addressed by the project]
# Literature Review: [Summarize the key findings from your research on existing solutions, technologies, or related work]
# Methodology: [Describe the approach and methodology used to design, implement, and test the project]
# System Design: [Explain the project's architecture, including its components, data structures, and algorithms]
# Implementation: [Detail the programming languages and techniques used to develop the project, including any libraries or frameworks]
# Testing and Validation: [Describe the testing process, including test cases, results, and any issues encountered]
# Results and Analysis: [Present the key findings and results of the project, analyzing their implications]
# Future Work: [Discuss potential future enhancements, improvements, or applications of the project]
# Contributions: [Highlight the project's contributions to your field of study]
# 3. Structure and Format:
# Target Length: [Specify the desired length of the report (e.g., 10-15 pages)]
# Chapter Structure: [Outline the desired structure of the report, including chapter headings and sub-sections]
# Visual Aids: [Specify any desired visual aids, such as diagrams, charts, or tables]
# 4. Additional Instructions:
# Tone and Style: [Indicate the desired tone and style of the report, such as formal, informal, academic, etc.]
# Keywords: [Include any specific keywords or phrases you want the generator to use]
# Constraints: [Specify any specific constraints, such as word count limitations, specific formats, or required elements]
# Example Prompt:
# Project Title: A Smart Home Automation System using AI
# Project Scope: Developing a smart home automation system capable of controlling lights, temperature, and appliances using AI-powered voice commands.
# Field of Study: Computer Science
# Target Audience: Academic audience, potential investors, and technology enthusiasts.
# Problem Statement: The current state of smart home technology often lacks a truly personalized and intelligent user experience. Users often need to interact with multiple devices and apps, leading to a fragmented and cumbersome experience.
# Methodology: The project will use a combination of Machine Learning algorithms for voice recognition, sensor data analysis, and device control.
# System Design: The system architecture will be based on a cloud-based platform, integrating voice recognition, sensor data processing, and device control capabilities.
# [Continue with other content and structure elements based on your project details]


class ProjectDetails(BaseModel):
    project_title: str
    project_scope: str
    field_of_study: str
    target_audience: str


def project_information(project_title):
    # Create the model
    generation_config = {
        "temperature": 1,
        "top_p": 0.95,
        "top_k": 64,
        "max_output_tokens": 200,
        "response_schema": content.Schema(
            type=content.Type.OBJECT,
            enum="[]",
            required="["project_title", "project_scope", "field_of_study", "target_audience"]",
        properties = {
        "project_title": content.Schema(
            type=content.Type.STRING,
        ),
        "project_scope": content.Schema(
            type=content.Type.STRING,
        ),
        "field_of_study": content.Schema(
            type=content.Type.STRING,
        ),
        "target_audience": content.Schema(
            type=content.Type.STRING,
        ),
    },
    ),
    "response_mime_type": "application/json",
    }


    model = genai.GenerativeModel(
        model_name="gemini-1.5-pro",
        generation_config=generation_config,
        # safety_settings = Adjust safety settings
        # See https://ai.google.dev/gemini-api/docs/safety-settings
        system_instruction="Project Information:\nProject Title: [Enter the project title]\nProject Scope: [Briefly describe the project's scope and objectives]\nField of Study: [Specify the academic field of the project]\nTarget Audience: [Who is the intended audience for this report?]",
    )
    Project_Prompt = f"""
    Project Title: {project_title}
    Project Scope: [Briefly describe the project's scope and objectives]
    Field of Study: [Specify the academic field of the project]
    Target Audience: [Who is the intended audience for this report?]
    """
    project_details_generated = project_generator.generate_content(
        Project_Prompt
    )
