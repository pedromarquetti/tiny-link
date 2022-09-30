import requests as req


def main():
    try:
        post = req.request(
            "POST",
            r"http://localhost:8080",
            data={
                "url": "a.e"
            },
        )

        print(post.text)
        print(post.status_code)
        get = req.request(
            "GET",
            r"http://localhost:8080",

        )
        print(get.text)
    except Exception as e:
        print(f"{e} occurred. Exiting")


if __name__ == '__main__':
    main()
