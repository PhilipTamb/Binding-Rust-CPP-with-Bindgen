# File: libvec/CMakeLists.txt
cmake_minimum_required(VERSION 3.18)#3.0
project(libvec CXX) #C

# use C++17
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

add_library(vec STATIC vec.cpp) #set the right name of .cpp file 

install(TARGETS vec DESTINATION .)