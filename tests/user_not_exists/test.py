#!/usr/bin/python3

import requests
import json
import random

def get_json(name, dir="data"):
    res = ""
    with open(dir + "/" + name) as f:
        res = f.read()
    res = json.loads(res)
    return res


host = "0.0.0.0"
port = "8081"

print("TestSuite User Not Exists")

add_user_json = get_json("register.json")
add_user_json["username"] = add_user_json["username"] + str(random.randint(0, 1000000))
resp = requests.post(f"http://{host}:{port}/auth/register", json=add_user_json)
assert(resp.status_code == 200)

print("test login")
login_json = get_json("login.json")
login_json["username"] = add_user_json["username"] + "asdasdas"
resp = requests.post(f"http://{host}:{port}/auth/login", json=login_json)
assert resp.status_code == 404, resp.status_code
