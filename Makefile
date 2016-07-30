# Manifest makefile for harald

.PHONY : start

start:
	cd libs/cloud && cargo run start
