"""
Install the Google AI Python SDK

$ pip install google-generativeai
$ pip install google.ai.generativelanguage
"""

"""
Generate Project Reports using Google Generative Language API (Gemini)
"""

import logging
import sys

import google.generativeai as genai
from google.ai.generativelanguage_v1beta.types import content
from google.colab import userdata  # If running in Google Colab

# Configure logging
logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)

# API Key Retrieval (using environment variables for better security)
api_key = userdata.get("GOOGLE_API_KEY")
if not api_key:
    logging.error("GOOGLE_API_KEY environment variable not set.")
    sys.exit(1)
genai.configure(api_key="AIzaSyC2fePo7G3IPTVf1Xy9Grvunu9WHkNSMd0")
# --- Initialize the Gemini model ONCE ---
model = genai.GenerativeModel(model_name="gemini-1.5-pro")

# Create the model
generation_config = {
    "temperature": 1,
    "top_p": 0.95,
    "top_k": 64,
    "max_output_tokens": 200,
    "response_schema": content.Schema(
        type=content.Type.OBJECT,
        enum="[]",
        required='["project_title", "project_scope", "field_of_study", "target_audience"]',
        properties={
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

chat_session = model.start_chat(
    history=[
        {
            "role": "user",
            "parts": [
                "Project Title: A Smart Home Automation System using AI",
            ],
        },
        {
            "role": "model",
            "parts": [
                "```json\n{\"field_of_study\": \"Computer Science, Artificial Intelligence\", \"project_scope\": \"Design and implement an AI-powered home automation system to control and optimize various aspects of a home environment, such as lighting, temperature, security, and entertainment.\", \"project_title\": \"A Smart Home Automation System using AI\", \"target_audience\": \"Academics, technology enthusiasts, potential investors\"} \n```",
            ],
        },
    ]
)

response = chat_session.send_message("INSERT_INPUT_HERE")

print(response.text)
