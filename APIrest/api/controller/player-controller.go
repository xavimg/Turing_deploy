package controller

import (
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
	"github.com/go-playground/validator"
	"github.com/xavimg/Turing/APIrest/api/entity"
	"github.com/xavimg/Turing/APIrest/api/service"
	"github.com/xavimg/Turing/APIrest/utils"
)

type VideoController interface {
	FindAll(ctx *gin.Context)
	Save(ctx *gin.Context) error
	Update(ctx *gin.Context) error
	Delete(ctx *gin.Context) error
	ShowAll(ctx *gin.Context)
}

type playerController struct {
	service service.PlayerService
}

var validate *validator.Validate

func NewConstructorController(service service.PlayerService) VideoController {
	validate = validator.New()
	validate.RegisterValidation("is-cool", utils.ValidateCoolUsername)
	return &playerController{
		service: service,
	}
}

func (c *playerController) FindAll(ctx *gin.Context) {
	players := c.service.FindAll()
	ctx.JSON(200, players)
}

func (c *playerController) Save(ctx *gin.Context) error {
	var player entity.Player
	err := ctx.ShouldBindJSON(&player)
	if err != nil {
		return err
	}
	err = validate.Struct(player)
	if err != nil {
		return err
	}
	c.service.Save(player)
	return nil

}
func (c *playerController) Update(ctx *gin.Context) error {
	var player entity.Player
	err := ctx.ShouldBindJSON(&player)
	if err != nil {
		return err
	}

	id, err := strconv.ParseUint(ctx.Param("id"), 0, 0)
	err = validate.Struct(player)
	if err != nil {
		return err
	}
	player.ID = id

	c.service.Update(player)
	return nil

}
func (c *playerController) Delete(ctx *gin.Context) error {
	var player entity.Player
	id, err := strconv.ParseUint(ctx.Param("id"), 0, 0)
	if err != nil {
		return err
	}
	player.ID = id

	c.service.Delete(player)
	return nil

}

func (c *playerController) ShowAll(ctx *gin.Context) {
	players := c.service.FindAll()
	data := gin.H{
		"titles":  "Player Page",
		"players": players,
	}
	ctx.HTML(http.StatusOK, "index.html", data)
}
