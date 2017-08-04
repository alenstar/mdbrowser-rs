Markdown browser
=================

The simple markdown browser based on webkit2 engine (linux gtk) 

### build

```shell
git submodule update --init --recursive
mkdir build
cd build
cmake -DCMARK_TESTS=OFF ../cmark
make 
cd ..
#设置编译时动态库查找路径
export LIBRARY_PATH=./build/src:./build/extensions:$LIBRARY_PATH
#设置运行时动态库查找路径
export LD_LIBRARY_PATH=./:$LD_LIBRARY_PATH
#构建release版
cargo build --release
```