from fastapi import APIRouter, Depends, status, HTTPException
from fastapi.security.oauth2 import OAuth2PasswordRequestForm
from sqlalchemy import or_
from sqlalchemy.orm import Session

from ..schemas import schemas

from ..models import models

from .. import database, utils, oauth2

router = APIRouter(tags=["Login"])


@router.post("/login", status_code=status.HTTP_200_OK, response_model=schemas.TokenResponse)
async def login(
        user_credentials: OAuth2PasswordRequestForm = Depends(),
        db: Session = Depends(database.get_db),
):
    # user_credentials.username = utils.decrypt_data(user_credentials.username)
    # user_credentials.password = utils.decrypt_data(user_credentials.password)
    # serial_number = utils.decrypt_data(serial_number)

    # if not serial_number:

    #     raise HTTPException(
    #         status_code=status.HTTP_400_BAD_REQUEST,
    #         detail="A serial number is required with each login",
    #     )

    user = (
        db.query(models.Users)
        .filter(
            or_(
                models.Users.username == user_credentials.username,
            )
        )
        .first()
    )

    if not user:
        utils.completeSeconds()
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN, detail="invalid credentials"
        )

    if not utils.verify(user_credentials.password, user.password):
        utils.completeSeconds()
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN, detail="invalid credentials"
        )

    # if not user.verified:
    #     raise HTTPException(
    #         status_code=status.HTTP_401_UNAUTHORIZED,
    #         detail="please authenticate your account first",
    #     )

    # if not user.active:
    #     login_confirm_token = device_verify.generate_token(user.email)
    #     await send_email.sendEmail(
    #         user.email,
    #         login_confirm_token,
    #         path="account/enable",
    #         subject="Enable Account",
    #         message="Looks like your account got temporarily disabled" if not arabic else "يبدو أنه تم تعطيل حسابك مؤقتًا",
    #         button_text="Enable Account" if not arabic else "تفعيل الحساب",
    #         arabic=arabic,
    #     )

    #     raise HTTPException(
    #         status_code=status.HTTP_403_FORBIDDEN,
    #         detail="please check you email to confirm your login",
    #     )

    # if not user.serial_number == serial_number:
    #     user.new_serial_number = serial_number
    #     db.commit()

    #     login_confirm_token = generate_token(user.email)
    #     await send_email.sendEmail(
    #         user.email,
    #         login_confirm_token,
    #         path="account/confirm-new-device",
    #         subject="Account Verify",
    #         message="You logged from another device, please confirm your login" if not arabic else "لقد قمت بتسجيل الدخول من جهاز آخر، يرجى تأكيد تسجيل الدخول الخاص بك",
    #         button_text="Verify Account" if not arabic else "التحقق من الحساب",
    #         arabic=arabic,
    #     )

    #     utils.completeSeconds()
    #     raise HTTPException(
    #         status_code=status.HTTP_403_FORBIDDEN,
    #         detail="please check you email to confirm your login",
    #     )

    access_token = oauth2.create_access_token(
        data={
            "user_id": user.id,
            "username": user_credentials.username,
        }
    )

    return {"access_token": access_token, "token_type": "bearer"}
