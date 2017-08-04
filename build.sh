#!/bin/bash

if [ -f build ] ; then
	mkdir build
	cd build
	cmake -DCMARK_TESTS=OFF ../cmark
	make -j4
	cd ..
fi
