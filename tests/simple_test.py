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

def test_json(name, res_json: dict):
    expected = get_json(name, dir="expected_data")
    for k,v in expected.items():
        assert k in res_json
        assert res_json.get(k) == v, f"expected: {v}, real: {res_json.get(k)}"


host = "0.0.0.0"
port = "8081"

print("test register new user")
add_user_json = get_json("register.json")
add_user_json["username"] = add_user_json["username"] + str(random.randint(0, 1000000))
resp = requests.post(f"http://{host}:{port}/auth/register", json=add_user_json)
assert(resp.status_code == 200)
assert(resp.cookies.get("jwt-token") is not None)
test_json("register.json", json.loads(resp.text))
token = resp.cookies.get("jwt-token")

print("test get profile")
resp = requests.get(f"http://{host}:{port}/auth/profile", cookies={"jwt-token": token})
assert(resp.status_code == 200)
test_json("get_profile1.json", json.loads(resp.text))

print("test update")
new_profile_json = get_json("update.json")
resp = requests.put(f"http://{host}:{port}/auth/profile", cookies={"jwt-token": token}, json=new_profile_json)
assert resp.status_code == 200, resp.status_code
test_json("update.json", json.loads(resp.text))

print("test login")
login_json = get_json("login.json")
login_json["username"] = add_user_json["username"]
resp = requests.post(f"http://{host}:{port}/auth/login", json=login_json)
assert(resp.status_code == 200)
test_json("login.json", json.loads(resp.text))
assert(resp.cookies.get("jwt-token") is not None)
token2 = resp.cookies.get("jwt-token")

print("test get profile")
resp = requests.get(f"http://{host}:{port}/auth/profile", cookies={"jwt-token": token2})
assert(resp.status_code == 200)
test_json("get_profile2.json", json.loads(resp.text))
