from locust import HttpUser, task, between
import json, requests

class Basic(HttpUser):
    wait_time = between(0.1, 1)

    def on_start(self):
        username = requests.get('http://127.0.0.1:5000/')
        response = self.client.post(url="/user", json={
            "name": username.json()['username'],
            "password": "password"
        })
        self.client.headers = {'Authorization': response.json()['jwt']}

    @task
    def home(self):
        self.client.get("/home")

    @task
    def create_then_delete_other_user(self):
        saved_headers = self.client.headers
        username = requests.get('http://127.0.0.1:5000/').json()['username']
        jwt = self.client.post(url="/user", json={
            "name": username,
            "password": "password"
        }).json()['jwt']
        self.client.delete(url="/user", json={
            "name": username,
        }, headers={'Authorization': jwt})
        self.client.headers = saved_headers
