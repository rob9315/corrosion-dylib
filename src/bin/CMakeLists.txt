cmake_minimum_required(VERSION 3.0.2)
project(bin)

find_package(catkin REQUIRED COMPONENTS
  cdylib
  dylib
  rlib
  staticlib
)

include(FetchContent)
FetchContent_Declare(
  Corrosion
  GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
  GIT_TAG v0.3
)
FetchContent_MakeAvailable(Corrosion)

catkin_package()

corrosion_import_crate(MANIFEST_PATH Cargo.toml
  PROFILE release
  CRATES ${PROJECT_NAME}
)

corrosion_install(
  TARGETS ${PROJECT_NAME}
  RUNTIME DESTINATION ${CATKIN_PACKAGE_BIN_DESTINATION}
)
