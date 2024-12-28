# import os
import platform
import subprocess
import sys


def install_brew():
    """Install Homebrew on macOS if it's not installed."""
    try:
        subprocess.check_call(
            [
                "/bin/bash",
                "-c",
                "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)",
            ]
        )
    except subprocess.CalledProcessError as e:
        print(f"Error installing Homebrew: {e}", file=sys.stderr)
        sys.exit(1)


def install_choco():
    """Install Chocolatey on Windows if it's not installed."""
    try:
        subprocess.check_call(
            [
                "powershell",
                "-Command",
                "Set-ExecutionPolicy Bypass -Scope Process -Force; "
                "[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; "
                "iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))",
            ]
        )
    except subprocess.CalledProcessError as e:
        print(f"Error installing Chocolatey: {e}", file=sys.stderr)
        sys.exit(1)


def install_package_manager(manager_command, install_function):
    """Check if a package manager is installed, if not install it."""
    try:
        subprocess.check_call(
            [manager_command, "--version"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
    except subprocess.CalledProcessError:
        print(f"{manager_command} not found, installing...")
        install_function()


def install_on_linux(packages):
    """Install packages on Linux based on the detected distribution."""
    distro = detect_linux_distro()
    if distro == "arch":
        subprocess.check_call(["sudo", "pacman", "-S", "--noconfirm"] + packages)
    elif distro in ["debian", "ubuntu"]:
        subprocess.check_call(["sudo", "apt-get", "update"])
        subprocess.check_call(["sudo", "apt-get", "install", "-y"] + packages)
    elif distro == "fedora":
        subprocess.check_call(["sudo", "dnf", "install", "-y"] + packages)
    elif distro == "opensuse":
        subprocess.check_call(["sudo", "zypper", "install", "-y"] + packages)
    else:
        print(f"Unsupported Linux distribution: {distro}", file=sys.stderr)
        sys.exit(1)


def install_on_windows(packages):
    """Install packages on Windows using Chocolatey."""
    install_package_manager("choco", install_choco)
    subprocess.check_call(["choco", "install", "-y"] + packages)


def install_on_mac(packages):
    """Install packages on macOS using Homebrew."""
    install_package_manager("brew", install_brew)
    subprocess.check_call(["brew", "install"] + packages)


def detect_linux_distro():
    """Detect the Linux distribution."""
    try:
        with open("/etc/os-release") as f:
            os_release = f.read().lower()
            if "arch" in os_release:
                return "arch"
            elif "ubuntu" in os_release or "debian" in os_release:
                return "debian"
            elif "fedora" in os_release:
                return "fedora"
            elif "opensuse" in os_release:
                return "opensuse"
            else:
                raise ValueError("Unsupported Linux distribution.")
    except FileNotFoundError:
        raise EnvironmentError("Could not detect the Linux distribution.")


def main():
    packages = ["plantuml", "graphviz", "pandoc"]
    current_os = platform.system().lower()

    if current_os == "linux":
        install_on_linux(packages)
    elif current_os == "windows":
        install_on_windows(packages)
    elif current_os == "darwin":
        install_on_mac(packages)
    else:
        print(f"Unsupported operating system: {current_os}", file=sys.stderr)
        sys.exit(1)

    print("Installation complete.")


if __name__ == "__main__":
    main()
