from core.database import init_db
from fastapi import FastAPI
from routers.bucket_router import router as bucket_router

app = FastAPI()

@app.on_event("startup")
def on_startup():
    init_db()

app.include_router(bucket_router)