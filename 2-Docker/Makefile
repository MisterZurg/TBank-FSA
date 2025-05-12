.PHONY: run
run:
	POSTGRES_USER="AlexanderZorkin" \
  	POSTGRES_PASSWORD="somnoynadno" \
  	POSTGRES_DB="tbank_fsa" \
  	POSTGRES_HOST="localhost" \
	POSTGRES_PORT="5432" \
	cargo run --manifest-path ./rust_app/Cargo.toml

deploy:
	docker compose -f ./docker-compose.yml rm && \
	docker compose -f ./docker-compose.yml build --no-cache && \
	docker compose -f ./docker-compose.yml up
