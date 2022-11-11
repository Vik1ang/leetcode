file(GLOB_RECURSE LINKED_LIST_DIR RELATIVE "${CMAKE_CURRENT_SOURCE_DIR}" "${CMAKE_CURRENT_SOURCE_DIR}/*.cpp")

foreach(src IN LISTS LINKED_LIST_DIR)
    get_filename_component(q ${src} NAME_WE)
    set(question linked_list_${q})
    message(STATUS ${question})
    add_executable(${q} ${src})
endforeach()