from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles

from app.database import engine
from app.models.models import Base
from app.routers import gen_proj, users, login, down_report, frontend

Base.metadata.create_all(bind=engine)

app = FastAPI(debug=True)

app.include_router(gen_proj.router)
app.include_router(users.router)
app.include_router(login.router)
app.include_router(down_report.router)
app.include_router(frontend.router)

app.mount("/static", StaticFiles(directory="."), name="static")