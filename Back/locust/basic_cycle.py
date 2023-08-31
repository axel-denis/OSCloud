from locust import HttpUser, task, between
import json, requests

class Basic(HttpUser):
    wait_time = between(1, 1)

    @task
    def login(self):
        self.client.post("/login", json={
            "name": "Arthur",
            "password": "mdp"
        })


"""
    @task
    def home(self):
        self.client.get("/home")

    def on_start(self):
        username = requests.get('http://127.0.0.1:5000/')
        response = self.client.post(url="/usernoopti", json={
            "name": username.json()['username'],
            "password": "password"
        })
        self.client.headers = {'Authorization': response.json()['jwt']}

    @task
    def create_then_delete_other_user_not_opti(self):
        saved_headers = self.client.headers
        username = requests.get('http://127.0.0.1:5000/').json()['username']
        try:
            jwt = self.client.post(url="/usernoopti", json={
                "name": username,
                "password": "password"
            }).json()['jwt']
        except:
            print("can't create")
        try:
            self.client.delete(url="/user", json={
                "name": username,
            }, headers={'Authorization': jwt})
        except:
            print("can't delete")
        self.client.headers = saved_headers


    @task
    def create_then_delete_other_user(self):
        saved_headers = self.client.headers
        username = requests.get('http://127.0.0.1:5000/').json()['username']
        try:
            jwt = self.client.post(url="/user", json={
                "name": username,
                "password": "password"
            }).json()['jwt']
        except:
            print("can't create")
        try:
            self.client.delete(url="/user", json={
                "name": username,
            }, headers={'Authorization': jwt})
        except:
            print("can't delete")
        self.client.headers = saved_headers
"""
