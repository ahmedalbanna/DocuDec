import os

from fastapi import APIRouter
from fastapi.responses import FileResponse

from ..converter.get_app_dir import getAppDirectory

router = APIRouter(tags=["Download"])


@router.get("/download_report/{filename}")
async def download_report(filename: str):
    file_path = os.path.join(getAppDirectory(),"user_reports","ahmed", filename)
    return FileResponse(
        file_path,
        media_type="application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        filename=filename,
    )
