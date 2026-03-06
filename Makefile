.PHONY: up down

up:
	$(MAKE) -C backend kratos-up
	$(MAKE) -C backend mailhog-up
	$(MAKE) -C backend app-up
	$(MAKE) -C frontend up

down:
	$(MAKE) -C backend kratos-down
	$(MAKE) -C backend mailhog-down
	$(MAKE) -C backend app-down
	$(MAKE) -C frontend down
