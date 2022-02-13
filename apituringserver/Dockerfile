FROM golang:alpine

WORKDIR /app

COPY . .

RUN go get -u github.com/lib/pq

RUN go build -o main .

CMD ["./main"]