# images

- docker.io/dltdojo/xidunku-xode-server:latest
- docker.io/dltdojo/xidunku-xode-server:k8scli
- docker.io/dltdojo/xidunku-xode-server:mdbook

# how to test

https://127.0.0.1:10443/?workspace=/home/coder/ws.json

```
$ skaffold dev --port-forward
$ skaffold dev -p k8scli --port-forward
```

# k8s ingress visual name host 

https://xode-server-127-0-0-1.nip.io/?workspace=/home/coder/ws.json


```
$ skaffold dev -p ing
```

# mdbook

port forward version

- HTTPS https://127.0.0.1:10443/?workspace=/home/coder/ws.json
- HTTP http://127.0.0.1:13000/

```
$ skaffold dev -p md --port-forward
```

ingress version with one 443 port service

- https://xode-server-127-0-0-1.nip.io/?workspace=/home/coder/ws.json
- https://mdbook-127-0-0-1.nip.io/

```
$ skaffold dev -p md-ing
```

websocket public address 

- https://github.com/rust-lang/mdBook/blob/554f29703f3034e77cab9b6bbc438e3b5ec3cfa3/src/cmd/serve.rs#L75

# dockerhub 

```
$ skaffold build -p dockerhub
$ skaffold build -p dockerhub-k8scli
$ skaffold build -p dockerhub-mdbook
```