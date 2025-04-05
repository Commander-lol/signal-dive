CURRENT_DIRECTORY=$(shell pwd)
DEV_FLAGS=-F bevy/file_watcher -F bevy/dynamic_linking

debug:
	cargo run $(DEV_FLAGS) -F desktop