package main

import (
	"io"
	"net/http"
	"os"

	"github.com/gin-gonic/gin"
	"github.com/xavimg/Turing/APIrest/api/controller"
	"github.com/xavimg/Turing/APIrest/api/middlewares"
	"github.com/xavimg/Turing/APIrest/api/repository"
	"github.com/xavimg/Turing/APIrest/api/service"
)

var (
	playerRepository repository.PlayerRepository = repository.NewConstructorRepository()
	playerService    service.PlayerService       = service.NewConstructorService(playerRepository)
	loginService     service.LoginService        = service.NewConstructorLogin()
	jwtService       service.JWTService          = service.NewJWTService()

	playerController controller.VideoController = controller.NewConstructorController(playerService)
	loginController  controller.LoginController = controller.NewLoginController(loginService, jwtService)
)

func main() {
	defer playerRepository.CloseDB()

	f, _ := os.Create("gin.log")
	gin.DefaultWriter = io.MultiWriter(f, os.Stdout)

	server := gin.New()
	server.Use(gin.Recovery(), middlewares.Logger())

	server.POST("/loginAdmin", func(c *gin.Context) {
		token := loginController.Login(c)
		if token == "" {
			c.JSON(http.StatusUnauthorized, nil)
			return
		}
		c.JSON(http.StatusOK, gin.H{
			"token": token,
		})
	})

	apiRoutes := server.Group("/api", middlewares.AuthorizeJWT())
	{
		apiRoutes.GET("/players", playerController.FindAll)

		apiRoutes.POST("/players", func(c *gin.Context) {
			err := playerController.Save(c)
			if err != nil {
				c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
				return
			}
			c.JSON(http.StatusOK, gin.H{"message": "Created succesfully!!"})

		})
		apiRoutes.PUT("/players/:id", func(c *gin.Context) {
			err := playerController.Update(c)
			if err != nil {
				c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
				return
			}
			c.JSON(http.StatusOK, gin.H{"message": "Updated susccesfully!!"})
		})
		apiRoutes.DELETE("/players/:id", func(c *gin.Context) {
			err := playerController.Delete(c)
			if err != nil {
				c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
				return
			}
			c.JSON(http.StatusOK, gin.H{"message": "Deleted susccesfully!!"})
		})
	}

	server.Run(":3000")
}
