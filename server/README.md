# Servers

This folder contains two components:

- **ClearService**: A public relay (C2 #1) that receives encrypted files via FTP and provides an API for data retrieval.
- **HiddenService**: A central server (C2 #2) operating over TOR that aggregates data from ClearService and hosts a ransom blog interface.
