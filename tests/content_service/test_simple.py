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

    with open(file_path, "rb") as f:
        content = f.read()
    return json.loads(content.decode("utf-8"))


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
    assert posts1["title"] == posts2["title"]
    assert posts1["description"] == posts2["description"]
    assert posts1["is_private"] == posts2["is_private"]
    assert posts1["tags"] == posts2["tags"]


def get_random_token():
    user = read_json("register.json")
    user["username"] = user["username"] + str(random.randint(0, 1000000))
    resp = send_post_to_gateway(user, handler="/auth/register")
    token = resp.cookies.get("jwt-token")
    assert token is not None
    return token


def create_post():
    input = read_json("create_message.json")
    token = get_random_token()
    resp = send_create_request(input, token)
    assert resp.status_code == 200, resp.text
    resp = json.loads(resp.json())
    assert_wallpost_content_equal(resp["post"]["content"], input)
    return resp, token


def create_post_with_token(token):
    input = read_json("create_message.json")
    resp = send_create_request(input, token)
    assert resp.status_code == 200, resp.text
    resp = json.loads(resp.json())
    assert_wallpost_content_equal(resp["post"]["content"], input)
    return resp

def add_comment(post_id, text, token):
    add_comment_resp = send_post_to_gateway({'text': text}, handler=f"/posts/comment/add/{post_id}", token=token)
    assert add_comment_resp.status_code == 200, add_comment_resp.text
    assert json.loads(add_comment_resp.json())['comment']['post_id'] == post_id
    return add_comment_resp


def test_create():
    create_post()


def test_delete():
    create_resp, token = create_post()
    post_id = create_resp["post"]["post_id"]
    print(create_resp)
    delete_resp = send_post_to_gateway(
        {}, handler=f"/posts/delete/{post_id}", token=token
    )
    assert delete_resp.status_code == 200, delete_resp.text
    assert json.loads(delete_resp.json()) == {"id": post_id}


def test_update():
    create_resp, token = create_post()
    post_id = create_resp["post"]["post_id"]
    print(create_resp)
    create_resp["post"]["content"]["title"] = "monkey"
    create_resp["post"]["content"]["description"] = "monkey description"
    print(create_resp)
    update_resp = send_post_to_gateway(
        create_resp["post"]["content"], handler=f"/posts/update/{post_id}", token=token
    )
    assert_wallpost_content_equal(
        create_resp["post"]["content"],
        json.loads(update_resp.json())["post"]["content"],
    )


def test_get():
    create_resp, token = create_post()
    post_id = create_resp["post"]["post_id"]
    get_resp = send_get_to_gateway(handler=f"/posts/get/{post_id}", token=token)
    assert get_resp.status_code == 200, get_resp.text
    assert_wallpost_content_equal(
        create_resp["post"]["content"], json.loads(get_resp.json())["post"]["content"]
    )


def test_gets():
    create_resp, token = create_post()
    create_resp2 = create_post_with_token(token)
    gets_resp1 = send_get_to_gateway(handler=f"/posts/gets?page=0&limit=1", token=token)
    print(json.loads(gets_resp1.json()))
    assert_wallpost_content_equal(
        create_resp["post"]["content"],
        json.loads(gets_resp1.json())["posts"][0]["content"],
    )
    gets_resp2 = send_get_to_gateway(handler=f"/posts/gets?page=1&limit=1", token=token)
    print(json.loads(gets_resp2.json()))
    assert_wallpost_content_equal(
        create_resp2["post"]["content"],
        json.loads(gets_resp2.json())["posts"][0]["content"],
    )


def test_add_like():
    create_resp, token = create_post()
    post_id = create_resp["post"]["post_id"]
    add_like_resp = send_post_to_gateway({}, handler=f"/posts/like/{post_id}", token=token)
    assert add_like_resp.status_code == 200, add_like_resp.text
    print(create_resp)
    assert json.loads(add_like_resp.json())['like']['post_id'] == post_id

def test_add_comment():
    create_resp, token = create_post()
    post_id = create_resp["post"]["post_id"]
    add_comment_resp = add_comment(post_id, "TEXT", token)
    assert add_comment_resp.status_code == 200

def test_get_comments():
    create_resp, token = create_post()
    post_id = create_resp["post"]["post_id"]
    text1 = "TEXT1"
    text2 = "TEXT228"
    add_comment_resp1 = add_comment(post_id, text1, token)
    comment1 = json.loads(add_comment_resp1.json())['comment']
    add_comment_resp2 = add_comment(post_id, text2, token)
    comment2 = json.loads(add_comment_resp2.json())['comment']
    get_comments_resp1 = send_get_to_gateway(handler=f"/posts/comment/get/{post_id}?page=0&limit=1", token=token)
    get_comments_resp2 = send_get_to_gateway(handler=f"/posts/comment/get/{post_id}?page=1&limit=1", token=token)
    assert get_comments_resp1.status_code == 200, get_comments_resp1.text
    assert get_comments_resp2.status_code == 200, get_comments_resp1.text
    comments1 = json.loads(get_comments_resp1.json())['comments']
    assert len(comments1) == 1
    comments2 = json.loads(get_comments_resp2.json())['comments']
    assert len(comments2) == 1
    assert set([comments1[0]['comment_id'], comments2[0]['comment_id']]) == set([comment1['comment_id'], comment2['comment_id']])
