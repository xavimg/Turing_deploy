package main

import (
	"fmt"
	"net/smtp"

	"github.com/gin-gonic/gin"
	"github.com/xavimg/Turing/apituringserver/config"
	"github.com/xavimg/Turing/apituringserver/controller"
	"github.com/xavimg/Turing/apituringserver/middleware"
	"github.com/xavimg/Turing/apituringserver/repository"
	"github.com/xavimg/Turing/apituringserver/service"
	"gorm.io/gorm"
)

var (
	db              *gorm.DB                   = config.SetupDatabaseConnection()
	userRepository  repository.UserRepository  = repository.NewUserRepository(db)
	adminRepository repository.AdminRepository = repository.NewAdminRepository(db)

	jwtService   service.JWTService   = service.NewJWTService()
	userService  service.UserService  = service.NewUserService(userRepository)
	authService  service.AuthService  = service.NewAuthService(userRepository)
	adminService service.AdminService = service.NewAdminService(adminRepository)

	authController  controller.AuthController  = controller.NewAuthController(authService, jwtService)
	userController  controller.UserController  = controller.NewUserController(userService, jwtService)
	adminController controller.AdminController = controller.NewAdminController(adminService)
)

func main() {

	r := gin.Default()

	email()

	r.MaxMultipartMemory = 8 << 20

	r.GET("/hello", func(c *gin.Context) {

		c.JSON(200, gin.H{
			"message": "Hello",
		})

	})

	// public routes
	authRoutes := r.Group("api/auth")
	{
		authRoutes.POST("/login", authController.Login)
		authRoutes.POST("/register", authController.Register)
		authRoutes.POST("/logout/:id", authController.Logout)
	}

	// private/tokenized routes
	userRoutes := r.Group("api/user", middleware.AuthorizeJWT(jwtService))
	{
		userRoutes.GET("/profile", userController.Profile)
		userRoutes.PUT("/update", userController.Update)
	}

	// admin Routes
	adminRoutes := r.Group("api/admin")
	{
		adminRoutes.PUT("/ban/:id", adminController.BanUser)
		adminRoutes.PUT("/unban/:id", adminController.UnbanUser)
		adminRoutes.POST("/newfeature", adminController.NewFeature)
	}

	r.Run(":3000")

}

func email() {
	// sender data
	from := "xavimg@blanquerna.url.edu" //ex: "John.Doe@gmail.com"
	password := "v6vpxdkd"              // ex: "ieiemcjdkejspqz"
	// receiver address
	toEmail := "xavimoyatesting@gmail.com" // ex: "Jane.Smith@yahoo.com"
	to := []string{toEmail}
	// smtp - Simple Mail Transfer Protocol
	host := "smtp.gmail.com"
	port := "587"
	address := host + ":" + port
	// message
	subject := "Subject: Our Golang Email\n"
	body := "our first email!"
	message := []byte(subject + body)
	// athentication data
	// func PlainAuth(identity, username, password, host string) Auth
	auth := smtp.PlainAuth("", from, password, host)
	// send mail
	// func SendMail(addr string, a Auth, from string, to []string, msg []byte) error
	err := smtp.SendMail(address, auth, from, to, message)
	if err != nil {
		fmt.Println("err:", err)
		return
	}
}
