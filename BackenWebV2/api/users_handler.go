package api

import "github.com/gofiber/fiber/v2"

func (w *WebServices) RegisterUserHandler(c *fiber.Ctx) error {
	var u RegisterUserBody
	err := c.BodyParser(&u)

	res, err := w.Services.users.RegisterUser(u)
	if err != nil {
		return fiber.NewError(400, "cannot register any user")
	}

	if len(res) == 0 {
		return c.JSON([]interface{}{})
	}
	return c.JSON(struct {
		Username string `json:"username"`
	}{
		Username: res,
	})

}
