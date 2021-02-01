#!/bin/bash
set -e

CMAKE=cmake-3.19.4

cd ~
wget https://github.com/Kitware/CMake/releases/download/v3.19.4/${CMAKE}.tar.gz && tar xf ${CMAKE}.tar.gz
cd ${CMAKE}
./bootstrap
make -j4
sudo make install
