package main

import (
	"github.com/gofiber/fiber"
	"github.com/xavimg/Turing/BackenWebV2/api"
)

func main() {
	app := fiber.New()
	api.SetupPlayerRoutes(app)
	_ = app.Listen("6845")
}
