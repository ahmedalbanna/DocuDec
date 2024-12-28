import re

from fastapi import (
    status,
    HTTPException,
    Depends,
    APIRouter,
)
from sqlalchemy.orm import Session

from ..schemas import schemas

from ..models import models

from .. import utils
from ..database import get_db

router = APIRouter(tags=["Users"])


@router.post(
    "/signup", status_code=status.HTTP_201_CREATED, response_model=schemas.UserResponse
)
async def create_user(
        user: schemas.UserCreate,
        db: Session = Depends(get_db),
):
    # user = utils.decrypt_data(user)
    # secret_key = user.secret_key
    # arabic = user.arabic
    # del user.secret_key
    # del user.arabic

    # if (
    #     db.query(models.SignedDevices)
    #     .filter(models.SignedDevices.serial_number == user.serial_number)
    #     .first()
    # ):
    #     utils.completeSeconds()
    #     raise HTTPException(
    #         status_code=status.HTTP_403_FORBIDDEN,
    #         detail="You cannot create any more accounts",
    #     )

    if not re.match(r"^[a-z0-9]{4,25}$", user.username):
        raise HTTPException(
            status_code=status.HTTP_406_NOT_ACCEPTABLE, detail="malformed username"
        )

    if db.query(models.Users).filter(models.Users.username == user.username).first():
        utils.completeSeconds()
        raise HTTPException(
            status_code=status.HTTP_306_RESERVED, detail="username is already reserved"
        )

    # if not re.match(r"^[a-zA-Z0-9_\.-]+@[a-zA-Z0-9]+.[a-z]{2,}$", user.email):
    #     raise HTTPException(
    #         status_code=status.HTTP_406_NOT_ACCEPTABLE, detail="malformed email"
    #     )

    if db.query(models.Users.username).filter(models.Users.username == user.username).first():
        utils.completeSeconds()
        raise HTTPException(
            status_code=status.HTTP_409_CONFLICT,
            detail="email is used for another account",
        )

    hashed_password = utils.hash(user.password)

    user.password = hashed_password
    new_user = models.Users(**user.model_dump())

    # Put the device inside the already signed up list to prevent it form creating more accounts
    # new_device = models.SignedDevices(
    #     username=user.username,
    #     email=user.email,
    #     laptop_user=user.laptop_user,
    #     serial_number=user.serial_number,
    # )

    # email_token = generate_token(user.email)
    # await send_email.sendEmail(
    #     user.email,
    #     email_token,
    #     path="account/verify",
    #     subject="Email Verification",
    #     message="Please verify your email" if not arabic else "يرجى التحقق من البريد الإلكتروني الخاص بك",
    #     button_text="Verify Email" if not arabic else "التحقق من البريد الإلكتروني",
    #     arabic=arabic
    # )

    db.add(new_user)
    # db.add(new_device)

    db.commit()

    db.refresh(new_user)
    # db.refresh(new_device)

    utils.completeSeconds()
    return new_user
