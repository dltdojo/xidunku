# https://hub.docker.com/r/codercom/code-server/tags
# 
FROM docker.io/codercom/code-server:2.1697-vsc1.39.2
#
# apt-get install
#
USER root
RUN apt-get update && apt-get install -y netcat-openbsd jq tree

#
# static curl
#
COPY --from=docker.io/dltdojo/xidunku-xhoami /curl /usr/local/bin/curl
COPY --from=docker.io/mikefarah/yq /usr/bin/yq /usr/local/bin/yq

USER coder
RUN mkdir -p /home/coder/.local/share/code-server/User/
ARG PATH_SETTINGS=/home/coder/.local/share/code-server/User/settings.json
COPY settings.json ${PATH_SETTINGS}

COPY ws.json /home/coder/
COPY docs /home/coder/docs
RUN sudo chown -R coder:coder /home/coder
#
# docker build -t foo .
# docker run -it --rm -v /tmp:/home/coder/project -p 8080:8080 foo 
# http://localhost:8080/?folder=/home/coder/project/
# 
# $ skaffold dev --port-forward
# http://127.0.0.1:18080/?workspace=/home/coder/ws.json
#
# https://github.com/cdr/code-server/blob/master/src/node/cli.ts
# ENTRYPOINT ["dumb-init", "code-server", "--host", "0.0.0.0", "--auth","none"]
ENTRYPOINT ["dumb-init", "--"]
CMD ["/bin/bash","-c","code-server --host 0.0.0.0 --port 8443 --cert"]