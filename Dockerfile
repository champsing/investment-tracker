FROM ubuntu:latest AS build
WORKDIR /build
COPY . .
RUN apt-get update \
&& apt-get -y install curl gnupg build-essential \
&& curl -sSL https://deb.nodesource.com/setup_20.x  | bash - \
&& apt-get -y install nodejs \
&& npm i -g vue-tsc vite \
&& npm i \
&& curl https://sh.rustup.rs -sSf | bash -s -- -y \
&& . "$HOME/.cargo/env" \
&& cargo build --release \
&& ls dist/ \
&& ls target/release/

FROM ubuntu:latest
WORKDIR /orike20
COPY --from=build /build/dist ./dist
COPY --from=build /build/target/release/orike20 ./

EXPOSE 8080
ENTRYPOINT [ "./orike20" ]
