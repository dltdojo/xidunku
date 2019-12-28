FROM docker.io/node:12-alpine
RUN apk update && apk add bash
WORKDIR /app
COPY package.json .
RUN  npm install
COPY *.js ./