#!/bin/bash

[ ! -d build ] && mkdir build

if [ -d build ] ; then
	cd build
	cmake -DCMARK_TESTS=OFF ..
	make -j4
	cd ..
	cp build/libcmark-wrapper.so .

	# for link
	export LIBRARY_PATH=./:$LIBRARY_PATH
	# for run
	export LD_LIBRARY_PATH=./:$LD_LIBRARY_PATH

	case "$#" in
		0)
			[ ! -f target/debug ] && mkdir -p target/debug 
			cp build/libcmark-wrapper.so target/debug
			cargo build
		;;
		1)
			if [ "%{1}" = "--release" ] ; then
				[ ! -f target/release ] && mkdir -p target/release
				cp build/libcmark-wrapper.so target/release
			else 
				[ ! -f target/debug ] && mkdir -p target/debug
				cp build/libcmark-wrapper.so target/debug
			fi
			cargo build ${1}
		;;
	esac
else
	echo "build cmark failed"
	exit 1;
fi
