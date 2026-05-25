from core.database import get_session
from fastapi import APIRouter, Depends
from models.bucket import Bucket
from sqlmodel import Session, select

router = APIRouter(prefix="/buckets", tags=["Buckets"])


@router.post("/")
def create_bucket(bucket: Bucket, session: Session = Depends(get_session)):
    session.add(bucket)
    session.commit()
    session.refresh(bucket)
    return bucket


@router.get("/")
def get_buckets(session: Session = Depends(get_session)):
    return session.exec(select(Bucket)).all()