package main

import (
	"authSystem/database"
	"authSystem/routes"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
)

func main() {
	database.Connection()

	app := fiber.New()

	// CORS ignore ports
	app.Use(cors.New(cors.Config{
		AllowCredentials: true, // Fronted can get the cookie
	}))

	routes.Setup(app)

	app.Listen(":3000")
}
