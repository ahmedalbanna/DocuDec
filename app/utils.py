import time

from passlib.context import CryptContext

from .config import settings

# Define Argon2 parameters
pwd_context = CryptContext(
    schemes=["argon2"],
    # You can customize Argon2 parameters here
    argon2__hash_len=32,  # Length of the resulting hash
    argon2__time_cost=3,  # Number of iterations
    argon2__memory_cost=65536,  # Memory usage in KB
    argon2__parallelism=4,  # Degree of parallelism
    deprecated="auto",
)

PEPPER = settings.pepper


# with open("private_key.pem", "rb") as key_file:
#     private_key = serialization.load_pem_private_key(key_file.read(), password=None)
# with open("public_key.pem", "rb") as key_file:
#     public_key = serialization.load_pem_public_key(key_file.read())

def hash(password: str):
    password_with_pepper = password + PEPPER
    return pwd_context.hash(password_with_pepper)


def verify(plain_password: str, hashed_password: str):
    password_with_pepper = plain_password + PEPPER
    return pwd_context.verify(password_with_pepper, hashed_password)


# def encryptData(payload: str):
#     encryted_data = public_key.encrypt( #type: ignore
#         payload.encode(),
#         padding.OAEP(
#             mgf=padding.MGF1(algorithm=hashes.SHA256()),
#             algorithm=hashes.SHA256(),
#             label=None,
#         ),
#     )
#     return encryted_data.hex()

# def decrypt_data(payload: str | BaseModel) -> Any:
#     try:
#         def decrypt_string(encrypted_str: str) -> str:
#             encrypted_data = bytes.fromhex(encrypted_str)
#             decrypted_data = private_key.decrypt( #type: ignore
#                 encrypted_data,
#                 padding.OAEP(
#                     mgf=padding.MGF1(algorithm=hashes.SHA256()),
#                     algorithm=hashes.SHA256(),
#                     label=None
#                 )
#             )
#             return decrypted_data.decode()

#         if isinstance(payload, str):
#             return decrypt_string(payload)
#         elif isinstance(payload, BaseModel):
#             decrypted_fields = {}
#             for field_name, field_value in payload.model_dump().items():
#                 if isinstance(field_value, str):
#                     try:
#                         decrypted_fields[field_name] = decrypt_string(field_value)
#                     except Exception:
#                         continue
#                 else:
#                     decrypted_fields[field_name] = field_value  # Handle non-string fields as needed
#             return payload.__class__(**decrypted_fields)
#         else:
#             raise TypeError("Payload must be a string, a Pydantic model")
#     except ValueError:
#         raise HTTPException(status_code=status.HTTP_406_NOT_ACCEPTABLE, detail="not accepted")

def completeSeconds():
    """
    Sleeps for the remaining miliseconds of a float.
    
    Adds an extra layer of protection for timing attacks

    """
    current_time = time.monotonic()
    seconds = 1.0

    splited_current_time = str(current_time).split(".")

    monotnoic_after_dot: float = float(str("0." + splited_current_time[1]))

    remained_miliseconds = seconds - monotnoic_after_dot

    time.sleep(remained_miliseconds)
