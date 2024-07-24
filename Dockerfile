FROM ubuntu:24.04
COPY ./target/release/server /bin/server
EXPOSE 8080
ENTRYPOINT ["server"]
CMD ["--bind-address", "0.0.0.0:8080"]
