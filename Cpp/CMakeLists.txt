cmake_minimum_required(VERSION 3.15)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++17 -fPIC -Wall -m64")

project("leetcode_cpp")

include_directories(include)

add_subdirectory(hash)
add_subdirectory(linked_list)