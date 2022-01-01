package api

import "github.com/gofiber/fiber"

func SetupPlayerRoutes(app *fiber.App) {
	s := start()
	grp := app.Group("/players")
	grp.Get("/", s.PlayerSearchHandler)
}
