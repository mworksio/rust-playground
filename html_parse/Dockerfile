FROM rust:1.23.0 as builder
WORKDIR /usr/src/html_parse
COPY . .
RUN cargo install --path .

#FROM debian:buster-slim
FROM mcr.microsoft.com/dotnet/core/sdk:3.1
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/html_parse /html_parse
COPY index.html /index.html
#CMD ["myapp"]

