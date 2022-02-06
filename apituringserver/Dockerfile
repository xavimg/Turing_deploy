FROM golang:alpine

WORKDIR /Go/src/github.com/xavimg/Turing/apituringserver

COPY go.mod go.sum ./
RUN go mod download

COPY . .

RUN go install github.com/cespare/reflex@latest

EXPOSE 3000

CMD reflex -g '*.go' go run src/main.go --start-service