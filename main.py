import uvicorn


if __name__ == "__main__":
    uvicorn.run("app.fastapi_app:app", port=8081, reload=True)
