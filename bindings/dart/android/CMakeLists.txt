set(zklinkVersion "0.0.1") # generated; do not edit

# Unlike the Windows & Linux CMakeLists.txt, this Android equivalent is just here
# to download the Android binaries into src/main/jniLibs/ and does not build anything.
# The binary download/extraction is difficult to do concisely in Groovy/Gradle,
# at least across host platforms, so we are just reusing our Linux/Windows logic.

# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

# Download the binaries if they are not already present.
set(LIBROOT "${CMAKE_CURRENT_SOURCE_DIR}/src/main/jniLibs")
set(ARCHIVE "${CMAKE_CURRENT_SOURCE_DIR}/v${zklinkVersion}.tar.xz")


# Extract the binaries, overriding any already present.
file(REMOVE_RECURSE ${LIBROOT})
file(MAKE_DIRECTORY ${LIBROOT})

file(MAKE_DIRECTORY ${LIBROOT}/armeabi-v7a)
file(DOWNLOAD
  "https://github.com/zkLinkProtocol/zklink_sdk/releases/download/dart_sdk_v${zklinkVersion}/armv7-linux-androideabi.tar.xz"
  ${ARCHIVE}
  TLS_VERIFY ON
)
execute_process(
  COMMAND ${CMAKE_COMMAND} -E tar xvf ${ARCHIVE} -C armeabi-v7a
  WORKING_DIRECTORY ${LIBROOT}
)

file(MAKE_DIRECTORY ${LIBROOT}/arm64-v8a)
file(DOWNLOAD
  "https://github.com/zkLinkProtocol/zklink_sdk/releases/download/dart_sdk_v${zklinkVersion}/aarch64-linux-android.tar.xz"
  ${ARCHIVE}
  TLS_VERIFY ON
)
execute_process(
  COMMAND ${CMAKE_COMMAND} -E tar xvf ${ARCHIVE} -C arm64-v8a
  WORKING_DIRECTORY ${LIBROOT}
)
