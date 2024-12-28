import asyncio
import logging
import tempfile
import uuid
from pathlib import Path

from fastapi import Depends, HTTPException
from fastapi.routing import APIRouter
from sqlalchemy.exc import SQLAlchemyError
from sqlalchemy.orm import Session

from ..models import models
from prompts import Project_Prompt
from .. import oauth2
from app.gemini.configure_gemeni import configureGemeni
from app.generator.chapter_prompts import get_chapter_prompts
from ..converter.generate_report import generate_report
from ..database import get_db
from app.gemini.misc import create_contact_information_markdown, generate_chapter_content
from ..schemas.schemas import ProjectDetails

logger = logging.getLogger(__name__)
logger.setLevel(logging.INFO)
# ... (logging handler setup remains the same) ...

router = APIRouter(tags=["Generation"])

project_generator, report_generator = configureGemeni()


async def _generate_chapter_content(chat, prompt):
    try:
        return await generate_chapter_content(chat, prompt)
    except Exception as e:
        logger.exception(f"Error generating chapter content: {e}")
        raise HTTPException(status_code=500, detail=f"Error generating chapter content: {e}")


async def _generate_project_report(project_details, current_user, chat):
    try:
        chapter_prompts = get_chapter_prompts()
        tasks = [_generate_chapter_content(chat, prompt) for prompt in chapter_prompts]
        responses = await asyncio.gather(*tasks)
        report_content = "".join([response.text for response in responses])
        return report_content
    except Exception as e:
        logger.exception(f"Error generating project report: {e}")
        raise HTTPException(status_code=500, detail=f"Error generating project report: {e}")


async def _save_project_to_db(db, new_project):
    try:
        db.add(new_project)
        db.commit()
        logger.info(f"Project saved to database: {new_project}")
    except SQLAlchemyError as e:
        db.rollback()
        logger.exception(f"Error saving project to database: {e}")
        raise HTTPException(status_code=500, detail=f"Error saving project to database: {e}")


@router.post("/generate_project/")
async def generate_project(
        project_details: ProjectDetails,
        db: Session = Depends(get_db),
        current_user=Depends(oauth2.get_current_user),
):
    """Generates a complete project report based on provided details."""

    logger.info(f"Generating project for user: {current_user.id}")

    try:
        # 1. Generate Project Details
        filled_project_prompt = Project_Prompt.replace(
            "[Enter the project title]", project_details.project_title
        )
        with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False) as temp_file:
            temp_file.write(filled_project_prompt)
            temp_file_project = temp_file.name

        project_details_generated = project_generator.generate_content(
            filled_project_prompt
        )

        # 2. Create Contact Information
        contact_info_md = create_contact_information_markdown(project_details)

        # 3. Start Chat Session
        chat = report_generator.start_chat(
            history=[
                {
                    "role": "user",
                    "parts": [contact_info_md, project_details_generated.text],
                }
            ]
        )

        # 4. Generate Chapter Content and Report
        report_content = await _generate_project_report(project_details, current_user, chat)

        # 5. Write Report Content to Temporary File
        with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False) as temp_file:
            temp_file.write(report_content)
            temp_file_name = temp_file.name

        # 6. Generate and Save Docx Report
        report_name = f"{current_user.username}_{project_details.project_title[:10]}_final_report_{uuid.uuid4()}"
        report_path = Path("./user_reports") / current_user.username / report_name
        report_path.parent.mkdir(parents=True, exist_ok=True)

        generate_report(filename=temp_file_name, reportname=report_name, user_username=current_user.username)

        # 7. Save Project to Database
        new_project = models.Projects(

            project_title=project_details.project_title,
            file_path=str(report_path),  # Store the full path
            project_owner=current_user.username,
        )
        await _save_project_to_db(db, new_project)

        return {"message": "Report generated successfully!", "filename": report_name}

    except Exception as e:
        logger.exception(f"Unexpected error during project generation: {e}")
        raise HTTPException(status_code=500, detail="An unexpected error occurred.")
    finally:
        # Clean up temporary files (even if exceptions occur)
        if Path(temp_file_project).exists():
            Path(temp_file_project).unlink()
        if Path(temp_file_name).exists():
            Path(temp_file_name).unlink()
