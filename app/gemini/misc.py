import asyncio

from app.config import settings
from app.schemas.schemas import ProjectDetails

MAX_CONCURRENT_REQUESTS = settings.max_concurrent_requests

semaphore = asyncio.Semaphore(MAX_CONCURRENT_REQUESTS)


async def generate_chapter_content(chat, prompt):
    """Asynchronously generates content for a chapter."""
    async with semaphore:
        response = await chat.send_message_async(prompt)
    return response


def create_contact_information_markdown(project_details: ProjectDetails) -> str:
    """Constructs the contact information section in Markdown format."""
    contact_info = "## Contact Information\n\n"
    for field, value in project_details.model_dump().items():
        if value is not None and field not in ["project_title", "project_description", "chapters"]:
            if field == "student_name":
                contact_info += f"**Student:**\n"
            elif field == "supervisor_name":
                contact_info += f"**Supervisor:**\n"
            elif field == "head_of_department":
                contact_info += f"**Head of Department:**\n"
            elif field == "acknowledgement":
                contact_info += f"**Acknowledgement:**\nThe student wishes to thank: {value}\n"
            else:
                contact_info += f"* **{field.replace('_', ' ').title()}:** {value}\n"

    return contact_info


