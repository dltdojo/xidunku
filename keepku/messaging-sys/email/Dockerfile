FROM debian:stretch-slim

# Install exim4
ENV DEBIAN_FRONTEND noninteractive
RUN set -ex; \
    apt-get update; \
    apt-get install -y netcat dumb-init dnsutils iputils-ping expect curl tree mailutils exim4-daemon-light; \
    apt-get clean

RUN adduser --gecos '' --disabled-password foo
RUN adduser --gecos '' --disabled-password bar

EXPOSE 25/tcp
ENTRYPOINT [ "dumb-init","--"]
CMD [ "exim", "-bdf", "-v", "-q30m" ]