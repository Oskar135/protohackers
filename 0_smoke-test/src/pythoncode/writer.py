import socket
import yaml
import os
from threading import Thread
import string
import random

def main():
    print("Python client running")
    MULTIPLE_THREADS=10
    with open(os.path.split(os.getcwd())[0] + "/src/tcpConfig.yml", "r") as stream:
        try:
            yml=yaml.safe_load(stream)
            config=yml["IP"]
            HOST=config["ipaddr"]
            PORT=config["port"]

        except yaml.YAMLError as e:
            print(e)
        
    ts=[]
    for i in range(MULTIPLE_THREADS):
        t = Thread(target=threaded_connection, args=(HOST,PORT))
        ts.append(t)
        t.start()
    for t in ts:
        t.join()

def get_random_byte_string(length):
    msg = ''.join([random.choice(string.ascii_letters) for i in range(length)]).encode('ascii')
    return msg

def threaded_connection(HOST,PORT):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        for i in range(1000):
            msg=get_random_byte_string(random.randint(10,1000))
            s.sendall(msg)
            data = s.recv(1024)
            data.decode("utf-8")
            print("Data recieved back from server (In Client): ", data)



if __name__=="__main__":
    main()