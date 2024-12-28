from sqlalchemy import Column, Integer, String, Boolean, DateTime, func
from sqlalchemy.orm import Mapped, mapped_column
from sqlalchemy.sql.expression import text
from sqlalchemy.sql.sqltypes import TIMESTAMP

from ..database import Base


class Users(Base):
    __tablename__ = "users"
    __table_args__ = {
        "mysql_charset": "utf8mb4",
        "mysql_collate": "utf8mb4_unicode_ci",
    }

    id = mapped_column(Integer, primary_key=True, autoincrement=True)  # More concise
    username = mapped_column(String(200), unique=True, nullable=False)
    password = mapped_column(String(500), nullable=False)
    phone_number = mapped_column(String(50), nullable=False) # Reduced length for phone numbers
    full_name = mapped_column(String(255), nullable=False)  # Reduced length
    download_left = mapped_column(Integer, default=5) #default value is set here
    verified = mapped_column(Boolean, default=False) #default value is set here
    premium = mapped_column(Boolean, default=False) #default value is set here
    active = mapped_column(Boolean, default=True)  #default value is set here
    created_at = mapped_column(DateTime(timezone=True), server_default=func.now()) #More efficient way to store timestamp


class Projects(Base):
    __tablename__ = "projects"
    __table_args__ = {
        "mysql_charset": "utf8mb4",
        "mysql_collate": "utf8mb4_unicode_ci",
    }
    id = mapped_column(Integer, primary_key=True, autoincrement=True) # More concise
    project_title = mapped_column(String(255), nullable=False) # Reduced length
    file_path = mapped_column(String(500), unique=True, nullable=False)
    project_owner = mapped_column(String(200), nullable=False) # Reduced length, using Foreign Key would be better
    created_at = mapped_column(DateTime(timezone=True), server_default=func.now())  #Added timestamp
