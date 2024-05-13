set(zklinkVersion "0.0.1") # generated; do not edit

# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

# Project-level configuration.
set(PROJECT_NAME "zklink")

# Download the binaries if they are not already present.
set(LIBROOT "${CMAKE_CURRENT_SOURCE_DIR}/v${zklinkVersion}")
set(ARCHIVE "${LIBROOT}.tar.gz")
if(NOT EXISTS ${ARCHIVE})
  file(DOWNLOAD
    "https://github.com/zkLinkProtocol/zklink_sdk/releases/download/dart_sdk_v${zklinkVersion}/x86_64-unknown-linux-gnu.tar.xz"
    ${ARCHIVE}
    TLS_VERIFY ON
  )
endif()

# Extract the binaries, overriding any already present.
file(REMOVE_RECURSE ${LIBROOT})
file(MAKE_DIRECTORY ${LIBROOT})
execute_process(
  COMMAND ${CMAKE_COMMAND} -E tar xvf ${ARCHIVE}
  WORKING_DIRECTORY ${LIBROOT}
)

# List of absolute paths to libraries that should be bundled with the plugin.
# This list could contain prebuilt libraries, or libraries created by an
# external build triggered from this build file.
set(zklink_bundled_libraries
  "${LIBROOT}/libzklink_sdk_dart.so"
  PARENT_SCOPE
)