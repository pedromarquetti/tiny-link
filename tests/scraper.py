import requests as req


def send_post(url: str):
    return req.request(
        "POST",
        r"http://localhost:8080",
        data={
            "url": "http://ti.jolo"
        },
    )


def send_get(path: str):
    return req.request(
        "GET",
        f"http://localhost:8080/{path}",
    )


def main():
    try:
        post_long_url = send_post("example.com")
        print(f"sending post req... response: {post_long_url.text}")
        print("sending get req... ")
        get_short_url = send_get(post_long_url.json()["url"]).json()
        print(f"""
            long url: {get_short_url["long_link"]}
            short link: {get_short_url["short_link"]}
        """)

    except Exception as e:
        print(f"{e} occurred. Exiting")


if __name__ == '__main__':
    main()
