package api

import "github.com/gofiber/fiber/v2"

func (w *WebServices) PlayerSearchHandler(c *fiber.Ctx) error {
	res, err := w.search.Search(PlayerFilter{
		Username: c.Query("username"),
		Level:    c.Query("level"),
	})

	if err != nil {
		return fiber.NewError(400, "cannot find any player")
	}

	if len(res) == 0 {
		return c.JSON([]interface{}{})
	}
	return c.JSON(res)
}
