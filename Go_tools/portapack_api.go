r := gin.Default()
r.POST("/havoc/deauth", func(c *gin.Context) {
	target := c.Query("bssid")
	runDeauthAttack(target) // Llama al firmware via serial/USB
	c.JSON(200, gin.H{"status": "🔥 Ataque iniciado!"})
})
r.Run() // Escucha en localhost:8080
