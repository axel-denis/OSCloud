#!/usr/bin/python3
from multiprocessing import Process
from clear_database import setup
from username_generator import start_app
import os
import time

def launch_locust():
    os.system('locust -f locust/basic_cycle.py --host=http://127.0.0.1:8888')

def launch_username_generator():
    start_app()

if __name__ == '__main__':
    setup()
    Process(target=launch_username_generator).start()
    time.sleep(1)
    Process(target=launch_locust).start()
