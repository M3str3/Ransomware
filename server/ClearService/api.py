import os
import sys
import time
import threading
import shutil
import sqlite3
import zipfile
import logging

from flask import Flask, jsonify, request, send_file, abort
from flask_cors import CORS

# Disable default Flask request logs so it doesn't constantly display incoming request addresses
logging.getLogger("werkzeug").setLevel(logging.ERROR)

# Configuration
app = Flask(__name__)
CORS(app, resources={r"/*": {"origins": "http://localhost:3000"}})

FTP_DIR = "ftp_upload"
PROCESSED_DIR = "ftp_processed"
DB_PATH = "database.db"

# Setup logging
logging.basicConfig(level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s")

# Retrieve API_KEY from command-line arguments or use default
AUTH_TOKEN = sys.argv[1] if len(sys.argv) > 1 else "m3str3"
logging.info(f"API_KEY: {AUTH_TOKEN}")

# Initialize the database
def init_db():
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute('''
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            description TEXT,
            price TEXT,
            published BOOLEAN
        )
    ''')
    conn.commit()
    conn.close()

init_db()

# ✅ Function to check if a folder has the correct structure
def is_valid_structure(folder_path):
    required_files = ["key", "file_tree.txt"]
    required_folders = ["data"]

    # Get folder contents
    folder_contents = os.listdir(folder_path)

    # Check that the required files are present
    if not all(file in folder_contents for file in required_files):
        return False

    # Check that the required folders are present
    if not all(os.path.isdir(os.path.join(folder_path, folder)) for folder in required_folders):
        return False

    return True

# 📂 Process a folder that is fully uploaded
def process_folder(folder_path):
    folder_name = os.path.basename(folder_path)

    # Insert into the database and get the assigned ID
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("INSERT INTO posts (name, description, price, published) VALUES (?, ?, ?, ?)",
              (folder_name, "No description yet", "No price yet", False))
    post_id = c.lastrowid
    conn.commit()
    conn.close()

    # Rename the folder with the ID and move it to the processed directory
    new_folder_path = os.path.join(PROCESSED_DIR, str(post_id))
    os.makedirs(PROCESSED_DIR, exist_ok=True)
    shutil.move(folder_path, new_folder_path)

    logging.info(f"✔ Processed folder: {folder_name} -> {post_id}")

# 🔄 Monitor ftp_upload to process complete folders
def monitor_ftp():
    while True:
        os.makedirs(FTP_DIR, exist_ok=True)

        for entry in os.listdir(FTP_DIR):
            folder_path = os.path.join(FTP_DIR, entry)

            # Check if it's a directory
            if os.path.isdir(folder_path):
                if is_valid_structure(folder_path):
                    logging.info(f"🟢 Folder ready to be processed: {folder_path}")
                    process_folder(folder_path)
                else:
                    logging.warning(f"⚠ Incomplete folder: {folder_path}")

        time.sleep(5)  # Check every 5 seconds

# Start monitoring in the background
threading.Thread(target=monitor_ftp, daemon=True).start()

@app.route('/posts/', methods=['GET'])
def posts():
    """Returns all posts in JSON format."""
    token = request.headers.get("Authorization")
    if not token or token != f"Bearer {AUTH_TOKEN}":
        abort(403, description="Access denied. Invalid token.")
    
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("SELECT id, name, description, price, published FROM posts")
    rows = c.fetchall()
    conn.close()
    
    posts_list = [{
        "id": row[0],
        "name": row[1],
        "description": row[2],
        "price": row[3],
        "published": bool(row[4])
    } for row in rows]
    
    return jsonify(posts_list)

@app.route('/download/<int:post_id>', methods=['GET'])
def download_post(post_id):
    """
    Allows downloading a processed folder as a ZIP file 
    if the correct token is provided.
    """
    token = request.headers.get("Authorization")
    if not token or token != f"Bearer {AUTH_TOKEN}":
        abort(403, description="Access denied. Invalid token.")

    folder_path = os.path.join(PROCESSED_DIR, str(post_id))
    if not os.path.exists(folder_path) or not os.path.isdir(folder_path):
        abort(404, description="Requested folder not found.")

    # Create a temporary ZIP file
    zip_filename = f"{post_id}.zip"
    zip_path = os.path.join(PROCESSED_DIR, zip_filename)
    
    with zipfile.ZipFile(zip_path, 'w', zipfile.ZIP_DEFLATED) as zipf:
        for root, _, files in os.walk(folder_path):
            for file in files:
                file_path = os.path.join(root, file)
                zipf.write(file_path, os.path.relpath(file_path, folder_path))
    
    return send_file(zip_path, as_attachment=True)

if __name__ == '__main__':
    # In debug mode for local testing
    app.run(host='0.0.0.0', port=5000, debug=False)
