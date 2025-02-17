import os
import time
import threading
import sqlite3
import logging
import requests
from flask_cors import CORS
from flask import Flask, jsonify, request

# Hide default Flask request logs so it doesn't repeatedly show incoming request addresses
logging.getLogger("werkzeug").setLevel(logging.ERROR)

# Flask configuration
app = Flask(__name__)
CORS(app)  # Enable CORS for all routes

# Database configuration
DB_PATH = "aggregator.db"

# Load server list from servers.txt
# Each line in the file has the format: URL|API_KEY
SERVERS = []
with open("servers.txt", "r") as f:
    servers_lines = f.read().splitlines()
    for line in servers_lines:
        if line.startswith("//"):
            # Skip comment lines
            continue
        parts = line.split("|")
        if len(parts) >= 2:
            SERVERS.append({"name": parts[0], "url": parts[1], "api_key": parts[2]})

# Logging setup
logging.basicConfig(level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s")

# Initialize the database
def init_db():
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute('''
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            server_id TEXT,
            remote_id INTEGER,
            name TEXT,
            description TEXT,
            price TEXT,
            published BOOLEAN,
            UNIQUE(server_id, remote_id) ON CONFLICT IGNORE
        )
    ''')
    conn.commit()
    conn.close()

init_db()

# Add CORS headers to allow access from any origin
@app.after_request
def after_request(response):
    response.headers.add("Access-Control-Allow-Origin", "*")
    response.headers.add("Access-Control-Allow-Headers", "Content-Type,Authorization")
    response.headers.add("Access-Control-Allow-Methods", "GET,PUT,POST,DELETE,OPTIONS")
    return response

# 📥 Download post data from a given server
def download_post_data(server, remote_id, post_name):
    download_url = f"{server['url']}/download/{remote_id}"
    headers = {"Authorization": f"Bearer {server['api_key']}"}
    try:
        response = requests.get(download_url, headers=headers, timeout=10)
        if response.status_code == 200:
            os.makedirs("downloads", exist_ok=True)
            # Create a filename based on server name and remote_id
            safe_server = server["name"]
            filename = f"downloads/{post_name}_{safe_server}_{remote_id}.zip"
            with open(filename, "wb") as f:
                f.write(response.content)
            logging.info(f"Downloaded post {remote_id} from {server['name']} to {filename}")
        else:
            logging.warning(f"Failed to download post {remote_id} from {server['name']}, code: {response.status_code}")
    except Exception as e:
        logging.error(f"Error downloading post {remote_id} from {server['name']}: {e}")

# 📦 Save posts to the local database and download new ones
def save_posts(server, posts):
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    
    for post in posts:
        remote_id = post["id"]
        name = post["name"]
        description = post["description"]
        price = post["price"]
        published = post["published"]

        # Check if the post already exists (by server and remote_id)
        c.execute("SELECT 1 FROM posts WHERE server_id = ? AND remote_id = ?", (server["name"], remote_id))
        exists = c.fetchone()
        if not exists:
            # Insert the new post
            c.execute('''
                INSERT INTO posts (server_id, remote_id, name, description, price, published)
                VALUES (?, ?, ?, ?, ?, ?)
            ''', (server["name"], remote_id, name, description, price, published))
            conn.commit()
            logging.info(f"New post found: {server['name']} id {remote_id}")
            # Download the full post data from the server
            download_post_data(server, remote_id, name)
    conn.close()
    logging.info(f"✅ Processed {len(posts)} posts from {server['name']}")

# 🔄 Periodically fetch posts from servers
def fetch_posts():
    while True:
        for server in SERVERS:
            try:
                headers = {"Authorization": f"Bearer {server['api_key']}"}
                response = requests.get(f"{server['url']}/posts/", headers=headers, timeout=5)
                
                if response.status_code == 200:
                    posts = response.json()
                    save_posts(server, posts)
                else:
                    logging.warning(f"⚠ Unable to fetch posts from {server['name']}. Code: {response.status_code}")

            except requests.RequestException as e:
                logging.error(f"❌ Error connecting to {server['name']}: {e}")

        time.sleep(10)  # Repeat every 10 seconds

# 🚀 API to retrieve locally stored posts
@app.route("/api/posts/", methods=["GET"])
def aggregated_posts():
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("SELECT server_id, remote_id, name, description, price, published FROM posts")
    rows = c.fetchall()
    conn.close()

    posts_list = [{
        "server_id": row[0],
        "remote_id": row[1],
        "name": row[2],
        "description": row[3],
        "price": row[4],
        "published": bool(row[5])
    } for row in rows]

    return jsonify(posts_list)

# 🚀 API to retrieve posts from a specific server
@app.route("/aggregated_posts/<server_id>", methods=["GET"])
def aggregated_posts_by_server(server_id):
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("SELECT remote_id, name, description, price, published FROM posts WHERE server_id = ?", (server_id,))
    rows = c.fetchall()
    conn.close()

    posts_list = [{
        "remote_id": row[0],
        "name": row[1],
        "description": row[2],
        "price": row[3],
        "published": bool(row[4])
    } for row in rows]

    return jsonify(posts_list)

# 🏃 Start monitoring in a separate thread
threading.Thread(target=fetch_posts, daemon=True).start()

if __name__ == "__main__":
    # Run the Flask application
    app.run(host="0.0.0.0", port=5001, debug=False)
