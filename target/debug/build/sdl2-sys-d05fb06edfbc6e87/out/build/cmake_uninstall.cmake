if (NOT EXISTS "/Users/jasonfu/Projects/gameboy_emulator/target/debug/build/sdl2-sys-d05fb06edfbc6e87/out/build/install_manifest.txt")
    message(FATAL_ERROR "Cannot find install manifest: \"/Users/jasonfu/Projects/gameboy_emulator/target/debug/build/sdl2-sys-d05fb06edfbc6e87/out/build/install_manifest.txt\"")
endif(NOT EXISTS "/Users/jasonfu/Projects/gameboy_emulator/target/debug/build/sdl2-sys-d05fb06edfbc6e87/out/build/install_manifest.txt")

file(READ "/Users/jasonfu/Projects/gameboy_emulator/target/debug/build/sdl2-sys-d05fb06edfbc6e87/out/build/install_manifest.txt" files)
string(REGEX REPLACE "\n" ";" files "${files}")
foreach (file ${files})
    message(STATUS "Uninstalling \"$ENV{DESTDIR}${file}\"")
    execute_process(
        COMMAND /opt/homebrew/bin/cmake -E remove "$ENV{DESTDIR}${file}"
        OUTPUT_VARIABLE rm_out
        RESULT_VARIABLE rm_retval
    )
    if(NOT ${rm_retval} EQUAL 0)
        message(FATAL_ERROR "Problem when removing \"$ENV{DESTDIR}${file}\"")
    endif (NOT ${rm_retval} EQUAL 0)
endforeach(file)

