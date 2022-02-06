package controller

import "github.com/gin-gonic/gin"

type AdminController interface {
	BanUser(ctx *gin.Context) error
}

type adminController struct {
}
