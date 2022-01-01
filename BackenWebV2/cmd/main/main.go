package main

import (
	"github.com/gofiber/fiber/v2"
	"github.com/xavimg/Turing/BackenWebV2/api"
)

func main() {

	app := fiber.New(fiber.Config{
		ErrorHandler: func(ctx *fiber.Ctx, err error) error {
			// Status code defaults to 500
			code := fiber.StatusInternalServerError
			var msg string
			// Retrieve the custom status code if it's an fiber.*Error
			if e, ok := err.(*fiber.Error); ok {
				code = e.Code
				msg = e.Message
			}

			if msg == "" {
				msg = "cannot process the http call"
			}

			// Send custom error page
			err = ctx.Status(code).JSON(internalError{
				Message: msg,
			})
			return nil
		},
	})

	api.SetupPlayerRoutes(app)
	api.SetupUserRoutes(app)
	_ = app.Listen(":3000")
}

type internalError struct {
	Message string `json:"message"`
}
