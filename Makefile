.PHONY: up
up:
	docker compose up --build -d

.PHONY: down
down:
	docker compose down

.PHONY: shell
shell:
	docker compose exec --user `id -u`:`id -g` develop bash
