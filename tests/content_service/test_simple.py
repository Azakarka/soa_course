import requests
import random
import json
import os
from pathlib import Path


host = "0.0.0.0"
port = "8081"


def read_json(filename):
    file_path = Path("data/" + filename)

    if not file_path.exists():
        raise FileNotFoundError(f"Файл {file_path} не найден")

    with open(file_path, 'rb') as f:
        content = f.read()
    return json.loads(content.decode('utf-8'))


def send_post_to_gateway(input, handler, token=None):
    uri = f"http://{host}:{port}{handler}"
    print(uri)
    response = requests.post(uri, json=input, cookies={"jwt-token": token})
    return response

def send_get_to_gateway(handler, token=None):
    uri = f"http://{host}:{port}{handler}"
    print(uri)
    response = requests.get(uri, cookies={"jwt-token": token})
    return response


def send_create_request(input, token):
    return send_post_to_gateway(input, handler="/posts/create", token=token)


def send_register_user():
    pass

def assert_wallpost_content_equal(posts1, posts2):
    assert posts1['title'] == posts2['title']
    assert posts1['description'] == posts2['description']
    assert posts1['is_private'] == posts2['is_private']
    assert posts1['tags'] == posts2['tags']


def get_random_token():
    user = read_json("register.json")
    user["username"] = user["username"] + str(random.randint(0, 1000000))
    resp = send_post_to_gateway(user, handler="/auth/register")
    token = resp.cookies.get("jwt-token")
    assert token is not None
    return token

def create_post():
    input = read_json('create_message.json')
    token = get_random_token()
    resp = send_create_request(input, token)
    assert resp.status_code == 200, resp.text
    resp  = json.loads(resp.json())
    assert_wallpost_content_equal(resp['post']['content'], input)
    return resp, token

def create_post_with_token(token):
    input = read_json('create_message.json')
    resp = send_create_request(input, token)
    assert resp.status_code == 200, resp.text
    resp  = json.loads(resp.json())
    assert_wallpost_content_equal(resp['post']['content'], input)
    return resp



def test_create():
    create_post()


def test_delete():
    create_resp, token = create_post()
    post_id = create_resp['post']['post_id']
    print(create_resp)
    delete_resp = send_post_to_gateway({}, handler=f"/posts/delete/{post_id}", token=token)
    assert delete_resp.status_code == 200, delete_resp.text
    assert json.loads(delete_resp.json()) == {'id': post_id}

def test_update():
    create_resp, token = create_post()
    post_id = create_resp['post']['post_id']
    print(create_resp)
    create_resp['post']['content']['title'] = "monkey"
    create_resp['post']['content']['description'] = "monkey description"
    print(create_resp)
    update_resp = send_post_to_gateway(create_resp['post']['content'], handler=f"/posts/update/{post_id}", token=token)
    assert_wallpost_content_equal(create_resp['post']['content'], json.loads(update_resp.json())['post']['content'])

def test_get():
    create_resp, token = create_post()
    post_id = create_resp['post']['post_id']
    get_resp = send_get_to_gateway(handler=f"/posts/get/{post_id}", token=token)
    assert_wallpost_content_equal(create_resp['post']['content'], json.loads(get_resp.json())['post']['content'])


def test_gets():
    create_resp, token = create_post()
    create_resp2 = create_post_with_token(token)
    gets_resp1  = send_get_to_gateway(handler=f"/posts/gets?page=1&limit=1", token=token)
    assert_wallpost_content_equal(create_resp['post']['content'], json.loads(gets_resp1.json())['posts'][0]['content'])
    gets_resp2  = send_get_to_gateway(handler=f"/posts/gets?page=2&limit=1", token=token)
    assert_wallpost_content_equal(create_resp2['post']['content'], json.loads(gets_resp2.json())['posts'][0]['content'])
