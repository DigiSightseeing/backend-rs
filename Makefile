APP_NAME := DigiTour

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  create            - Create and run the DigiTour Docker container."
	@echo "  stop              - Stop the DigiTour Docker container."
	@echo "  clean             - Stop and remove the DigiTour Docker container."

.PHONY: create
create:
	@sudo docker run --name $(APP_NAME) \
		-p 5432:5432 \
		-e POSTGRES_USER=user \
		-e POSTGRES_PASSWORD=password \
		-e POSTGRES_DB=DigiTourLocal \
		-d postgres:latest
	@echo "$(APP_NAME) Docker container created and started."

.PHONY: start
start: create
	@sudo docker start $(APP_NAME)
	@echo "$(APP_NAME) Docker container started."

.PHONY: stop
stop:
	@sudo docker stop $(APP_NAME)
	@echo "$(APP_NAME) Docker container stopped."

.PHONY: clean
clean: stop
	@sudo docker rm $(APP_NAME)
	@echo "$(APP_NAME) Docker container removed."
