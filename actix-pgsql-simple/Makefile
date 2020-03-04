run:
	@echo -e '\e[1;31mRunning...\e[0m'
	@cd docker && docker-compose up --build
	@echo -e '\e[1;31mDone\e[0m'

start:
	@echo -e '\e[1;31mStarting...\e[0m'
	@cd docker && docker-compose start
	@echo -e '\e[1;31mDone\e[0m'

stop:
	@echo -e '\e[1;31mStopping...\e[0m'
	@cd docker && docker-compose stop
	@echo -e '\e[1;31mDone\e[0m'

destroy:
	@echo -e '\e[1;31mDestroying...\e[0m'
	@cd docker && docker-compose down
	@echo -e '\e[1;31mDone\e[0m'

ssh:
	@docker exec -ti casbin-actix-pgsql-app /bin/bash

check:
	@cargo fmt
	@cargo clippy -- -D warnings
