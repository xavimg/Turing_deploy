package main

import (
	"github.com/labstack/echo/v4"
	"github.com/xavimg/Turing/golangserver/route"
)

func main() {
	var server = echo.New()

	// route.SetupMiddleware(server)
	route.SetupRoutes(server)

	server.Logger.Fatal(server.Start(":3000"))
}
