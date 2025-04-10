# defined the targets:  build, install and update and marked them as .PHONY
# you can run the following commands, and they will execute the associated actions:

.PHONY: build install update

# Packages Ferris is using
DEPS = libnotify-bin dunst

build:
	@echo " -> Building ..."
	@cargo build --release

install: build  # Run the build target before installing
	@echo " -> Installing system dependencies..."
	@for pkg in $(DEPS); do \
		dpkg -s $$pkg >/dev/null 2>&1 || { \
			echo " -> Installing missing package: $$pkg"; \
			sudo apt-get install -y $$pkg; \
			dunst & \
		}; \
	done
	@echo " -> Deploying the latest binary ..."
	@mkdir -p ~/bin
	@cp -f ./target/release/ferris ~/bin
	@chmod 755 ~/bin/ferris

update:
	@echo " -> Updating to the latest versions of Penrose, Penrose_UI crates ..."
	@cargo update -p penrose
	@cargo update -p penrose_ui
