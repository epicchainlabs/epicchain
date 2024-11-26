FROM debian:stable-slim

# Install the apt-get packages
RUN apt-get update
RUN apt-get install -y libicu-dev libleveldb-dev screen

COPY ./dist /opt/epicchain-cli
RUN ln -s /opt/epicchain-cli/epicchain-cli /usr/bin
