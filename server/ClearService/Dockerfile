FROM python:3.10

ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1
ARG API_KEY
ENV API_KEY=$API_KEY

RUN apt-get update && apt-get install -y \
    build-essential \
    python3-dev \
 && rm -rf /var/lib/apt/lists/*

RUN pip install --no-cache-dir flask flask-cors pyftpdlib

RUN mkdir /app
WORKDIR /app
RUN mkdir ftp_upload ftp_processed

RUN touch database.db

COPY api.py /app/api.py
RUN echo $API_KEY 
RUN echo $API_KEY > /app/API_KEY
EXPOSE 2121
EXPOSE 5000

CMD sh -c "python3 -m pyftpdlib --port 2121 --write --directory ftp_upload & \
           python3 api.py $API_KEY"
