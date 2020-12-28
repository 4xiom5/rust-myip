```bash
$ docker build -t rust-myip .
$ docker run --rm -d --network host rust-myip
```
The server will run on port 8000