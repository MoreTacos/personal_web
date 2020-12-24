#!/bin/zsh
cd ./blog/blog-compile/
cargo b --release
cp ./target/release/personal-web ../
cd ./..
./personal-web > ../../blog.html
cd ./../../blog
for i in ../build/blog/blogs/*.md; do
    a=$(basename $i)
    b="${a%.*}.html"
    cat "../build/blog/top.txt" > $b
    pandoc $i >> $b
    cat "../build/blog/bot.txt" >> $b
done
