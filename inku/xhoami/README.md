# Embed htdoc htmls into a Go executable

- http://localhost:80/pub/

```
$ statik -src=htdocs/pub
$ skaffold build
$ skaffold build -p dockerhub
$ docker images | grep xhoami | awk '{print $1":"$2,$7}' | head -n 1
dltdojo/xidunku-xhoami:latest 10MB
```

# curl

```
$ docker run -it --rm --entrypoint=/curl docker.io/dltdojo/xidunku-xhoami -V
curl 7.67.0 (x86_64-pc-linux-gnu) libcurl/7.67.0 OpenSSL/1.1.1d nghttp2/1.39.2
Release-Date: 2019-11-06
Protocols: dict file ftp ftps gopher http https imap imaps pop3 pop3s rtsp smb smbs smtp smtps telnet tftp 
Features: AsynchDNS HTTP2 HTTPS-proxy IPv6 Largefile NTLM NTLM_WB SSL TLS-SRP UnixSockets
```

# swagger-ui

- http://localhost:80/pub/swagger-ui/

```dockerfile
COPY --from=docker.io/dltdojo/xidunku-xhoami /go/xhoami/xhoami /xhoami
COPY your-static-path-with-api-yaml /static/api.yaml
ENTRYPOINT ["/xhoami"]
```