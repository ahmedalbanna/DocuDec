import os
import google.generativeai as genai
import asyncio

genai.configure(api_key="AIzaSyDIlivnAT09Y8D5AIZKU0fnWOXJIY0dP4s")

generation_config = {
    "temperature": 1,
    "top_p": 0.95,
    "top_k": 40,
    "max_output_tokens": 8192,
    "response_mime_type": "text/plain",
}

model = genai.GenerativeModel(
    model_name="gemini-2.0-flash-exp",
    generation_config=generation_config,
    system_instruction="""Instructions for various agents categorized by functionality:

I. Core Functional Areas:
- Project Planning
- Graduation Project Reporting
- Project Management
- Documentation Generation
- Research
- PlantUML Diagram Generation

II. Individual Agent Instructions & Styles:

A. Project Planning Agent:
- Define Project Title (concise, engaging)
- Set Objective (SMART)
... other commands

B. Graduation Report Generator Agent:
- Generate Title Page (formal)
... other commands

... other agents (C through T) with commands and styles

III. Key Styles & Common Threads:
- Clarity
- Conciseness
- Accuracy
- Structure
- Detail
- Formality (academic contexts)
- Visuals (PlantUML)
- Automation
- User-Centricity""" # shortened instructions
)



commands_and_prompts = [
    {"command": "Project Planning Agent", "prompt": "You are a project planning agent... implementation and achievement of goals."},
    {"command": "Define Project Title", "prompt": "Determine a concise and clear project title... stakeholders and team members."},
    # ... other commands and prompts
]

async def generate_responses():
    response = await model.generate_content([f"command: {item['command']}\nprompt: {item['prompt']}\noutput: " for item in commands_and_prompts])
    await asyncio.sleep(5) # sleep for 5 seconds
    print(response.text)

asyncio.run(generate_responses())


