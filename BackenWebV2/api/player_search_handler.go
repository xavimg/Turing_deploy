package api

import "github.com/gofiber/fiber"

func (w *WebServices) PlayerSearchHandler(c *fiber.Ctx) {
	res, err := w.s.search.Search(PlayerFilter{})

	if err != nil {
		err = fiber.NewError(400, "cannot bring players")
		c.Next(err)
	}
	c.JSON(res)
}
