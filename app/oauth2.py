from datetime import datetime, timedelta, timezone

from fastapi import Depends, status, HTTPException
from fastapi.security import OAuth2PasswordBearer
from jose import JWTError, jwt
from sqlalchemy.orm import Session

from .schemas import schemas

from .models import models

from . import database
from .config import settings

outh2_scheme = OAuth2PasswordBearer(tokenUrl="login", auto_error=True)

SECRET_KEY = settings.secret_key
ALGORITHM = settings.algorithm
ACCESS_TOKEN_EXPIRE_MINUTES = settings.access_token_expire_minutes


def create_access_token(data: dict):
    to_encode = data.copy()

    expire = datetime.now(timezone.utc) + timedelta(minutes=ACCESS_TOKEN_EXPIRE_MINUTES)
    to_encode.update({"exp": expire})

    encoded_jwt = jwt.encode(to_encode, SECRET_KEY, algorithm=ALGORITHM)

    return encoded_jwt


def verify_access_token(token: str, credentials_exception):
    try:
        payload = jwt.decode(token, SECRET_KEY, algorithms=ALGORITHM)
        id = payload.get("user_id")
        if id is None:
            raise credentials_exception
        token_data = schemas.TokenData(id=id)
    except JWTError:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED, detail="please get a new token"
        )

    return token_data


async def getANewTokenFromToken(token):
    try:
        payload = jwt.decode(token, SECRET_KEY, algorithms=ALGORITHM)
        username = payload.get("username")
        # password = payload.get("password")
        id = payload.get("user_id")
        if id is None:
            return {"detail": "expired toke, login again"}
        else:
            return create_access_token({"user_id": id, "username": username})

    except JWTError:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED, detail="please get a new token"
        )


def get_current_user(
        token: str = Depends(outh2_scheme), db: Session = Depends(database.get_db)
):
    credential_exception = HTTPException(
        status_code=status.HTTP_401_UNAUTHORIZED,
        detail="Could not validate credentials",
        headers={"WWW-Authenticate": "Bearer"},
    )

    token_ = verify_access_token(token, credential_exception)

    # user = db.query(model.User).filter(model.User.id == token.id).first()
    user = (
        db.query(models.Users.id, models.Users.username, models.Users.created_at)
        .filter(models.Users.id == token_.id)
        .first()
    )
    return user
