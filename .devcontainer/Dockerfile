FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld curl wget


# Install postgresql-client-14
RUN echo "Installing postgresql-client-14" \
   && echo "deb [signed-by=/usr/share/keyrings/postgresql-keyring.gpg] http://apt.postgresql.org/pub/repos/apt/ bullseye-pgdg main" | sudo tee /etc/apt/sources.list.d/postgresql.list \
   && curl -fsSL https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo gpg --dearmor -o /usr/share/keyrings/postgresql-keyring.gpg


RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get install postgresql-client-14 -y

RUN apt-get autoremove -y && apt-get clean -y

RUN curl -fsSL -o /usr/local/bin/dbmate https://github.com/amacneil/dbmate/releases/latest/download/dbmate-linux-amd64 \
   && chmod +x /usr/local/bin/dbmate

# Install Node.js v18
RUN curl -sL https://deb.nodesource.com/setup_18.x | bash - \
   && apt-get install -y nodejs

RUN cd /usr/local/bin \ 
   && wget -qO- https://github.com/thedodd/trunk/releases/download/v0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

RUN curl -sSL https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-x86_64-unknown-linux-gnu.deb -o /tmp/cargo-watch.deb \
   && dpkg -i /tmp/cargo-watch.deb \
   && rm /tmp/cargo-watch.deb

RUN mkdir /storage && chown -R vscode:vscode /storage
