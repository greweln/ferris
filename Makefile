# defined the targets:  build, install and update and marked them as .PHONY
# you can run the following commands, and they will execute the associated actions:

.PHONY: build install update


build: update
	@echo " -> Building ..."
	@cargo build --release

install: build  # Run the build target before installing
	@echo " -> Deploying the latest binary ..."
	@mkdir -p ~/bin
	@cp -f ./target/release/ferris ~/bin
	@chmod 755 ~/bin/ferris

update:
	@echo " -> Updating Penrose, Penrose_UI crates ..."
	@cargo update -p penrose
	@cargo update -p penrose_ui

	
