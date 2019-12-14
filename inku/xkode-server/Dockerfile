FROM docker.io/dltdojo/xidunku-xode-server:latest
USER root
#
# kubectl download 
# https://kubernetes.io/docs/tasks/tools/install-kubectl/#install-kubectl-binary-with-curl-on-linux
# 
ENV KUBECTL_VERSION=v1.15.6
RUN curl -sLO https://storage.googleapis.com/kubernetes-release/release/${KUBECTL_VERSION}/bin/linux/amd64/kubectl && \
    chmod +x ./kubectl && mv ./kubectl /usr/local/bin/kubectl && kubectl version --client
#
# skaffold
#
RUN curl -sLo skaffold https://storage.googleapis.com/skaffold/releases/v1.0.1/skaffold-linux-amd64 && \
    chmod +x skaffold && sudo mv skaffold /usr/local/bin
USER coder
COPY docs /home/coder/docs
RUN sudo chown -R coder:coder /home/coder