import json

def save_history_json(history, filename="chat_history.json"):
    with open(filename, "w") as f:
        json.dump(history, f, indent=4)  # Save with pretty printing

def load_history_json(filename="chat_history.json"):
    try:
        with open(filename, "r") as f:
            return json.load(f)
    except FileNotFoundError:
        return []  # Return an empty list if the file doesn't exist

import sqlite3  # Example using SQLite

def save_history_db(user_id, role, message, db_file="chat_history.db"):
    conn = sqlite3.connect(db_file)
    c = conn.cursor()
    c.execute("INSERT INTO history (user_id, role, message) VALUES (?, ?, ?)", (user_id, role, message))
    conn.commit()
    conn.close()


def load_history_db(user_id, db_file="chat_history.db"):
    conn = sqlite3.connect(db_file)
    c = conn.cursor()
    c.execute("SELECT role, message FROM history WHERE user_id = ?", (user_id,))
    history = []
    for role, message in c.fetchall():
        history.append({"role": role, "parts": [message]})
    conn.close()
    return history


def view_history_json():
    for turn in load_history_json():
        print(f"Role: {turn['role']}")
        for part in turn['parts']:
            print(f"  Part: {part}")


def view_history_db(user_id):
    for turn in load_history_db(user_id=user_id):
        print(f"Role: {turn['role']}")
        for part in turn['parts']:
            print(f"  Part: {part}")

    