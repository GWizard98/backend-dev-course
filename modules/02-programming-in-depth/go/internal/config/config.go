package config

import (
	"os"
	"strconv"
)

type Settings struct {
	Port     int
	Env      string
	LogLevel string
}

func Load() Settings {
	port := 3000
	if v := os.Getenv("PORT"); v != "" {
		if p, err := strconv.Atoi(v); err == nil {
			port = p
		}
	}
	env := os.Getenv("APP_ENV")
	if env == "" {
		env = "dev"
	}
	log := os.Getenv("LOG_LEVEL")
	if log == "" {
		log = "info"
	}
	return Settings{Port: port, Env: env, LogLevel: log}
}
