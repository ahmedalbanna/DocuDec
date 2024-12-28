import google.generativeai as genai
import logging
import time

# Configure logging
logger = logging.getLogger(__name__)
logger.setLevel(logging.INFO)
handler = logging.StreamHandler()
formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')
handler.setFormatter(formatter)
logger.addHandler(handler)

GOOGLE_API_KEY = "AIzaSyDIlivnAT09Y8D5AIZKU0fnWOXJIY0dP4s"  # Replace with your actual key
genai.configure(api_key=GOOGLE_API_KEY)

check_plantuml_syntax = """ ## Check PlantUML Syntax Error For All Diagrams:

This guide will walk you through how to check for syntax errors in your PlantUML diagrams, ensuring they are correctly formatted and ready to be rendered.  Only correct syntax errors; do not alter the diagram's meaning or structure."""


def code_generator(plantuml_code, max_retries=3, retry_delay=2):
    """Uses Gemini to attempt to fix syntax errors in PlantUML code."""
    if not isinstance(plantuml_code, str):
        raise TypeError("plantuml_code must be a string.")
    if not plantuml_code.strip().startswith("@start"): #basic validation ;; Input Validation: The function doesn't validate the input plantuml_code. It should check if it's a string and potentially check for basic PlantUML structure (e.g., presence of @startuml and @enduml). Invalid input could lead to unexpected behavior or errors in Gemini.
        raise ValueError("Invalid PlantUML code: missing '@startuml' or '@startgantt'.")

    #Prompt Engineering: The prompt itself could be improved. Consider adding more specific instructions, examples, or context to guide Gemini more effectively. For instance, specify that only syntax should be corrected and that the overall structure and meaning shouldn't be changed.
    prompt = f"""
    The following PlantUML code has a syntax error:

    ```plantuml
    {plantuml_code}
    ```

    Please provide the corrected PlantUML code.  Only correct syntax errors; do not alter the diagram's meaning or structure.
    ```plantuml
    """

    gemini_model = genai.GenerativeModel(
        model_name="models/gemini-1.5-flash-8b-exp-0827", #Consider a different model if needed The choice of the specific Gemini model (models/gemini-1.5-flash-8b-exp-0827) might need justification. Different models have different strengths and weaknesses. Consider if a more suitable model exists for PlantUML syntax correction. Experimentation with different models might be needed.
        system_instruction=check_plantuml_syntax,
        safety_settings=[
            {"category": "HARM_CATEGORY_HARASSMENT", "threshold": "block_none"},
            {"category": "HARM_CATEGORY_DANGEROUS_CONTENT", "threshold": "block_none"},
            {"category": "HARM_CATEGORY_HATE_SPEECH", "threshold": "block_none"},
        ],
        generation_config=genai.types.GenerationConfig(
            candidate_count=1,
            stop_sequences=[],
            max_output_tokens=1000,
            top_p=0.9,
            top_k=3,
            temperature=0.3,
        ),
    )

    retries = 0
    while retries < max_retries:
        try:
            response = gemini_model.generate_content(prompt)
            if response.text:
                fixed_code = response.text.strip().removeprefix("```plantuml").removesuffix("```").strip()
                if fixed_code.startswith("@start") and any(s in fixed_code for s in ["@enduml", "@endgantt"]): #very basic validation ;;Output Validation: After receiving the response from Gemini, the function extracts fixed_code. It should validate that fixed_code is actually valid PlantUML before returning it. A simple check might be to see if it contains @startuml and @enduml, but a more robust approach might involve using a PlantUML linter (if one exists).


                    logger.info("Gemini successfully fixed PlantUML syntax.")
                    return fixed_code
                else:
                    logger.warning("Gemini returned invalid PlantUML code.")
                    return None
            else:
                logger.warning("Gemini returned an empty response.")
                return None

        except Exception as e:
            logger.exception(f"Error calling Gemini API (attempt {retries + 1}/{max_retries}): {e}")
            retries += 1
            time.sleep(retry_delay)

    logger.error(f"Failed to fix PlantUML syntax after {max_retries} retries.")
    return None

