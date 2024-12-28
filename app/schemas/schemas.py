from datetime import datetime
from typing import Optional

from pydantic import BaseModel


class ProjectDetails(BaseModel):
    project_title: str
    # background: str
    # objectives: str
    # problem: str
    # significance: str
    student_name: str
    department: str
    university: str
    date: str
    email_address: str
    phone_number: str
    student_id_number: str
    supervisor_name: str
    supervisor_email: str
    supervisor_phone: str
    head_of_department: str
    acknowledgement: str


class UserCreate(BaseModel):
    username: str
    full_name: str
    phone_number: str
    password: str


class UserResponse(BaseModel):
    id: int
    # email: EmailStr
    created_at: datetime

    class Config:
        from_attributes = True


class TokenResponse(BaseModel):
    access_token: str
    token_type: str


class TokenData(BaseModel):
    id: Optional[int] = None
