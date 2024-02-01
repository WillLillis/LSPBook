#!/usr/bin/env bash
set -e

tectonic -o build/ build_book.tex 
makeindex build/book.idx -s indexstyle.ist
biber main/book.tex
tectonic -o build/ build_book.tex 
xpdf build/build_book.pdf
