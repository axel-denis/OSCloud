#!/usr/bin/python3
import psycopg2
from dotenv import dotenv_values

def setup():
    config = dotenv_values(".env")

    conn = psycopg2.connect(config['DATABASE_URL'])
    cur = conn.cursor()
    a = cur.execute("DELETE FROM users WHERE name LIKE 'locust-users-%';")
    conn.commit()
    cur.close()
    conn.close()
    print("database cleaned :D")

if __name__ == '__main__':
    setup()
