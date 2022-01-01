package api

import "github.com/gofiber/fiber/v2"

func SetupPlayerRoutes(app *fiber.App) {
	s := start()
	grp := app.Group("/players")
	grp.Get("/", s.PlayerSearchHandler)
}

func SetupUserRoutes(app *fiber.App) {
	s := start()
	grp := app.Group("/users")
	grp.Post("/", s.RegisterUserHandler)
}
