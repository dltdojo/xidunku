FROM alpine:3.10 AS curl-builder
RUN apk add gcc make musl-dev openssl-dev nghttp2-static nghttp2-dev ca-certificates
RUN wget https://curl.haxx.se/download/curl-7.67.0.tar.xz &&\
    tar xvfJ curl-7.67.0.tar.xz && cd curl-* && \
    ./configure --disable-shared --with-ca-fallback --with-nghttp2 &&\
    make curl_LDFLAGS=-all-static &&\ 
    make install
#
# xhoami web server
#
FROM docker.io/golang:1.13-alpine as go-builder

RUN apk --no-cache --no-progress add git ca-certificates tzdata make \
    && update-ca-certificates \
    && rm -rf /var/cache/apk/*

WORKDIR /go/xhoami

# Download go modules
COPY go.mod .
COPY go.sum .
RUN GO111MODULE=on GOPROXY=https://proxy.golang.org go mod download
COPY . .
RUN CGO_ENABLED=0 go build -a --installsuffix cgo --ldflags="-s" -o xhoami
# Create a minimal container to run a Golang static binary
FROM scratch

COPY --from=go-builder /usr/share/zoneinfo /usr/share/zoneinfo
COPY --from=go-builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=go-builder /go/xhoami/xhoami .
COPY --from=curl-builder /usr/local/bin/curl .
COPY htdocs/static /static

#
# [rakyll/statik: Embed files into a Go executable](https://github.com/rakyll/statik)
#
# DEPRECATED copy static files 
# COPY htdocs ./htdocs

ENTRYPOINT ["/xhoami"]
EXPOSE 80