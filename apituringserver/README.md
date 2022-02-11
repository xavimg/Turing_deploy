1. First of all you have to run the server.
2. Once server is running you can start making calls to this end-points:

   // public routes route will be something like this in PostMan: http://localhost:8080/auth/login
   authRoutes := r.Group("api/auth")
   {
   authRoutes.POST("/login", authController.Login)
   authRoutes.POST("/register", authController.Register)
   authRoutes.POST("/logout/:id", authController.Logout)
   }

   // private/tokenized routes. Route will be something like this in PostMan: http://localhost:8080/user/profile
   // this time as you can see in the line 14, to make call the function, first you need to validate a middleware authorization, you can get this using the middleware from auth request.
   userRoutes := r.Group("api/user", middleware.AuthorizeJWT(jwtService))
   {
   userRoutes.GET("/profile", userController.Profile)
   userRoutes.PUT("/update", userController.Update)
   }

   // admin Routes. Same logic as others
   adminRoutes := r.Group("api/admin")
   {
   adminRoutes.PUT("/ban/:id", adminController.BanUser)
   adminRoutes.PUT("/unban/:id", adminController.UnbanUser)
   }
