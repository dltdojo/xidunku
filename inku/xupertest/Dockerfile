FROM docker.io/node:12-alpine

RUN apk add --no-cache openssl jq tree bash tzdata && rm -rf /var/cache/apk/*

#
# base on [containous/whoami: Tiny Go webserver that prints os information and HTTP request to output](https://github.com/containous/whoami)
#
COPY --from=docker.io/dltdojo/xidunku-xhoami /xhoami /xhoami
#
# static curl
#
COPY --from=docker.io/dltdojo/xidunku-xhoami /curl /usr/local/bin/curl

# [mikefarah/yq: yq is a portable command-line YAML processor](https://github.com/mikefarah/yq)
COPY --from=docker.io/mikefarah/yq /usr/bin/yq /usr/local/bin/yq

ADD package.json /app/
WORKDIR /app
RUN npm install
ADD . /app

ENTRYPOINT ["/xhoami"]
EXPOSE 80