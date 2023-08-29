#!/usr/bin/python3
from flask import Flask, jsonify
app = Flask(__name__)

count = 0

def start_app():
    app.run(port=5000)

@app.route("/")
def username():
    global count

    count = count+1

    return jsonify({'username':"locust-users-{}".format(str(count))})

if __name__ == "__main__":
    start_app()
