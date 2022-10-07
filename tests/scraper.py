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
        print("sending get req")
        print(send_post("example.com").text)
        print("sending get req... ")
        print(send_get("teste").text)

    except Exception as e:
        print(f"{e} occurred. Exiting")


if __name__ == '__main__':
    main()
