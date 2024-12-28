from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from ..converter.get_app_dir import getAppDirectory

from fastapi import APIRouter
import os.path

router = APIRouter(tags=["Home"])
@router.get("/", response_class=HTMLResponse)
async def home(request: Request):
    app_dir = getAppDirectory()
    templates_dir = os.path.join(app_dir, "templates")
    templates = Jinja2Templates(directory=templates_dir)
    return templates.TemplateResponse("index.html", {"request": request})
