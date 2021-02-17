#!/bin/bash

# ./build
cd ./blog/blog-main/

# ./build/blog/blog-main
cargo b --release
cp ./target/release/blog-main-script ./../
cd ./..

# ./build/blog
./blog-main-script > ../../blog/index.html
cd ./blog-sub

# ./build/blog/blog-sub
cargo b --release
cp ./target/release/blog-sub-script ../
cd ./../../../blog

# ./blog
for i in ../build/blog/blogs/*.md; do
    a=$(basename $i)
    b="${a%.*}.html"
    rm $b
    pandoc $i >> "./temp.html"
    cp ./../build/blog/blog.hbs ./
    echo "${a%.*}" | ./../build/blog/blog-sub-script >> $b
    rm "./temp.html"
    rm ./blog.hbs
done
