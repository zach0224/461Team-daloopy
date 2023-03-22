test:
	./run URLs.txt

build:
	./run build

install:
	./run install

source:
	source ~/.bash_profile

env:
	nano ~/.bash_profile

.PHONY: test, build