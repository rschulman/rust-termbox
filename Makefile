all: build

build: nsf  

nsf: nsf/libtermbox.a

nsf/libtermbox.a:
	mkdir -p nsf
	(cd nsf && curl -L https://github.com/nsf/termbox/tarball/master | tar -xz)
	(cd nsf/nsf-termbox* && ./waf configure && ./waf)
	rm -f nsf/libtermbox.a
	mv nsf/nsf-termbox*/build/src/libtermbox.a nsf/libtermbox.a
	rm -rf nsf/nsf-termbox*

clean:
	rm -rf nsf
	rm -f libtermbox*.so
	rm -f demo
	rm -f doc/*.html

.PHONY: clean doc nsf examples
