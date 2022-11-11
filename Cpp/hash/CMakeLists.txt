file(GLOB_RECURSE HASH_DIR RELATIVE "${CMAKE_CURRENT_SOURCE_DIR}" "${CMAKE_CURRENT_SOURCE_DIR}/*.cpp")

foreach(src IN LISTS HASH_DIR)
    get_filename_component(q ${src} NAME_WE)
    set(question hash_${q})
    message(STATUS ${question})
    add_executable(${q} ${src})
endforeach()