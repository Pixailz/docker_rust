# CONFIG
SHELL				:= /bin/bash
DOCKER_COMPOSE		:= docker compose# -f ./srcs/docker-compose.yaml
CURL				:= curl -L -\#
ENV_FILE			:= .env

# VOLUMES DIR
SHARE_BASE			:= ./Shared
SHARE_DIR			:= rust \
					   srcs

SHARE_DIR			:= $(addprefix $(SHARE_BASE)/,$(SHARE_DIR))

# PACKAGE TO DOWNLOAD

PACKAGES			:= $(WORDPRESS_PACKAGE) $(PORTAINER_PACKAGE) $(WP_CLI_PACKAGE)

# UTILS
MKDIR				= \
$(shell [ -f $(1) ] && rm -f $(1)) \
$(shell [ ! -d $(1) ] && mkdir -p $(1))

ifeq ($(findstring fre,$(MAKECMDGOALS)),fre)
RE_STR				:= --no-cache
endif

ifneq ($(ENTRY),)
ENTRYPOINT			:= --entrypoint $(ENTRY)
endif

RE_STR				?=
TARGET				?= rust

ESC					:=\x1b[
R					:=$(ESC)38;5;196m
G					:=$(ESC)38;5;112m
B					:=$(ESC)38;5;27m
O					:=$(ESC)38;5;208m

PRI					:=$(G)
SEC					:=$(O)
RST					:=$(ESC)00m

export				U_UID=$(shell id -u)
export				G_UID=$(shell id -g)

# RULES
.PHONY:				up run build kill exec re clean fclean $(SHARE_DIR)

run:				build
	$(DOCKER_COMPOSE) run -it $(ENTRYPOINT) $(TARGET)

up:					build
	$(DOCKER_COMPOSE) up $(TARGET)

build:				$(PACKAGES) $(SHARE_DIR) $(ENV_FILE)
	$(DOCKER_COMPOSE) build $(TARGET) $(RE_STR)

kill:
	$(DOCKER_COMPOSE) kill $(TARGET)

exec:
	$(DOCKER_COMPOSE) exec -it $(TARGET) ash

re:					clean run

fre:				fclean run

clean:
	sudo rm -rf $(SHARE_BASE)

fclean:				kill clean
	docker system prune -af
	docker stop $(shell docker ps -qa) 2>/dev/null; true
	docker rm $(shell docker ps -qa) 2>/dev/null; true
	docker rmi $(shell docker images -qa) 2>/dev/null; true
	docker volume rm $(shell docker volume ls -q) 2>/dev/null; true
	docker network rm $(shell docker network ls -q) 2>/dev/null; true

reset_env:
	cp -f ./.env{.template,}

$(ENV_FILE):		reset_env

$(SHARE_DIR):
	$(call MKDIR,$(@))
