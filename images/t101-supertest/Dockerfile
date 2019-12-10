FROM docker.io/node:12-alpine

RUN apk add --no-cache openssl curl jq tree bash tzdata && rm -rf /var/cache/apk/*
COPY --from=docker.io/containous/whoami /whoami /whoami

ADD package.json /app/
WORKDIR /app
RUN npm install
ADD . /app

ENTRYPOINT ["/whoami"]
EXPOSE 80