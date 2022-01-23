package user

import (
	"github.com/labstack/echo/v4"
)

type Handler struct {
	service UserService
}

func NewHandler(s UserService) *Handler {
	return &Handler{
		service: s,
	}
}

func (h *Handler) AllUserHandler(ctx echo.Context) error {

	res, err := h.service.AllUser()
	if err != nil {
		return err
	}
	return ctx.JSON(200, res)
}

// func (h *Handler) FindByIdHandler(ctx *gin.Context) error {

// }
