import google.generativeai as genai

from app.config import settings
from app.prompts import System_Instructions, Project_Prompt

# Use settings directly instead of intermediate variables
MAX_OUTPUT_TOKENS = settings.max_output_tokens
API_KEY = settings.google_api_key  # Renamed for clarity
genai.configure(api_key=API_KEY)


# Create a common generation config function
def _create_generation_config(max_tokens):
    return genai.types.GenerationConfig(
        stop_sequences=[],
        max_output_tokens=max_tokens,
        top_p=0.95,
        top_k=64,
        temperature=1,
    )


# Create a common Gemini model initialization function
def _create_gemini_model(model_name, generation_config, system_instructions):
    return genai.GenerativeModel(
        model_name=model_name,
        system_instruction=system_instructions,
        safety_settings=[
            {"category": "HARM_CATEGORY_HARASSMENT", "threshold": "block_none"},
            {"category": "HARM_CATEGORY_DANGEROUS_CONTENT", "threshold": "block_none"},
            {"category": "HARM_CATEGORY_HATE_SPEECH", "threshold": "block_none"},
        ],
        generation_config=generation_config,
    )


def configureGemeni():
    # Use the helper functions to create the models
    project_generator_config = _create_generation_config(max_tokens=8192)
    report_generator_config = _create_generation_config(max_tokens=MAX_OUTPUT_TOKENS)

    Project_MODEL_NAME = "models/gemini-1.5-flash-8b-exp-0827"  # Define model name as a constant
    Report_MODEL_NAME = "models/gemini-1.5-flash-8b-exp-0827"  # Define model name as a constant

    project_generator = _create_gemini_model(Project_MODEL_NAME, project_generator_config,Project_Prompt)
    report_generator = _create_gemini_model(Report_MODEL_NAME, report_generator_config,System_Instructions)

    return project_generator, report_generator
