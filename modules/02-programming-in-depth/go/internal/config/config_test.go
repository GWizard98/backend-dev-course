package config

import (
	"os"
	"testing"
)

func TestDefaults(t *testing.T) {
	clearenv()
	s := Load()
	if s.Port != 3000 || s.Env != "dev" || s.LogLevel != "info" {
		t.Fatalf("unexpected defaults: %+v", s)
	}
}

func TestFromEnv(t *testing.T) {
	clearenv()
	os.Setenv("PORT", "7777")
	os.Setenv("APP_ENV", "test")
	os.Setenv("LOG_LEVEL", "debug")
	s := Load()
	if s.Port != 7777 || s.Env != "test" || s.LogLevel != "debug" {
		t.Fatalf("unexpected values: %+v", s)
	}
}

func clearenv() {
	os.Unsetenv("PORT")
	os.Unsetenv("APP_ENV")
	os.Unsetenv("LOG_LEVEL")
}
