FROM golang:1.14

WORKDIR /src/hello
COPY . .
RUN go get -v .

WORKDIR /go
CMD ["./bin/hello"]
