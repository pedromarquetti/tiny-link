import requests as req
post = req.request(
    "POST",
    r"http://localhost:8080",
    data={
        "url": " "
    },


)
print(post.text)
print(post.status_code)
