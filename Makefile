# Manifest makefile for harald

.PHONY : start

start:
	cd libs/server && cargo run start
