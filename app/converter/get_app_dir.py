import os
import sys
from functools import cache


@cache
def getAppDirectory() -> str:
    """
    Gets the script or the
    executable path
    """
    if getattr(sys, "frozen", False):
        # Running in a bundle
        bundle_dir = os.path.dirname(sys.executable)
    else:
        # Running in a normal Python environment
        bundle_dir = os.path.dirname(os.path.abspath(__file__))
    if bundle_dir.endswith("app/converter"):
        bundle_dir = bundle_dir[: -len("app/converter")]

    if bundle_dir.endswith("converter"):
        bundle_dir = bundle_dir[: -len("converter")]

    return bundle_dir
