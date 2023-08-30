#!/usr/bin/python3
from multiprocessing import Process
from clear_database import setup
from username_generator import start_app
import os
import time
import signal

def launch_locust():
    os.system('locust -f locust/basic_cycle.py --host=http://127.0.0.1:8888')

def launch_username_generator():
    start_app()

def handler(signum, frame):
    print("Ctrl-c was pressed.", end="", flush=True)
    generator.terminate()
    locust.terminate()
    exit(1)

if __name__ == '__main__':
    signal.signal(signal.SIGINT, handler)

    setup()
    generator = Process(target=launch_username_generator)
    locust = Process(target=launch_locust)
    generator.start()
    time.sleep(1)
    locust.start()
