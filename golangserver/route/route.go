package route

import (
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/xavimg/Turing/golangserver/config/db"
	"github.com/xavimg/Turing/golangserver/feature/user"
	"github.com/xavimg/Turing/golangserver/repository"
	"github.com/xavimg/Turing/golangserver/repository/mysql"
)

func SetupMiddleware(e *echo.Echo) {
	e.Use(middleware.RequestID())
}

func SetupRoutes(e *echo.Echo) {

	// Repository
	var (
		gormDb = db.NewMysqlDB()

		userRepo repository.UserRepository = mysql.NewUserRepository(gormDb)
	)

	// Service
	var (
		userService = user.NewUserService(userRepo)
	)

	// Handlers / Controllers
	var (
		userHandler = user.NewHandler(userService)
	)

	// Routes
	g := e.Group("/admin")
	g.GET("/users", userHandler.AllUserHandler)
}
