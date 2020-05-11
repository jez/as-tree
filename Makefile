# This Makefile is not meant to be a complete representation of everything that
# can be built. It's just here for people who expect to be able to
#   make && make install
# to install something from source. See the README.md for more information.

.PHONY: build
build:
	./bazel build //src:as-tree -c opt

prefix := $(HOME)/.local

.PHONY: install
install: build
	mkdir -p "$(prefix)/bin" && \
		install bazel-bin/main/as_tree "$(prefix)/bin/as-tree"
