# Makefile for downloader

# 默认目标
.PHONY: all clean build release help

# 获取当前系统架构
UNAME_S := $(shell uname -s)

# 根据系统设置可执行文件扩展名
ifeq ($(OS),Windows_NT)
	EXT := .exe
else
	EXT :=
endif

# 主要目标
all: build

# 构建开发版本
build:
	cargo build

# 构建发布版本
release:
	cargo build --release

# 安装到系统
install: release
	@echo "Installing downloader..."
ifeq ($(UNAME_S),Linux)
	sudo cp target/release/downloader /usr/local/bin/
endif
ifeq ($(UNAME_S),Darwin)
	sudo cp target/release/downloader /usr/local/bin/
endif
ifeq ($(OS),Windows_NT)
	copy target\release\downloader.exe C:\Windows\System32\
endif

# 运行测试
test:
	cargo test

# 清理构建文件
clean:
	cargo clean

# 构建所有平台版本
cross-build: 
	./build-release.sh

# 显示帮助信息
help:
	@echo "Available targets:"
	@echo "  all          - Build development version (default)"
	@echo "  build        - Build development version"
	@echo "  release      - Build release version"
	@echo "  install      - Install to system"
	@echo "  test         - Run tests"
	@echo "  clean        - Clean build files"
	@echo "  cross-build  - Build for all platforms"
	@echo "  help         - Show this help"