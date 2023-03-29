cmake_minimum_required(VERSION 3.0.2)
project(cdylib)

find_package(catkin REQUIRED)

include(FetchContent)
FetchContent_Declare(
  Corrosion
  GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
  GIT_TAG v0.3
)
FetchContent_MakeAvailable(Corrosion)

###################################
## catkin specific configuration ##
###################################
## The catkin_package macro generates cmake config files for your package
## Declare things to be passed to dependent projects
## INCLUDE_DIRS: uncomment this if your package contains header files
## LIBRARIES: libraries you create in this project that dependent projects also need
## CATKIN_DEPENDS: catkin_packages dependent projects also need
## DEPENDS: system dependencies of this project that dependent projects also need
catkin_package(
#  INCLUDE_DIRS include
 LIBRARIES cdylib
#  CATKIN_DEPENDS other_catkin_pkg
#  DEPENDS system_lib
)

corrosion_import_crate(MANIFEST_PATH Cargo.toml
  PROFILE release
  CRATES ${PROJECT_NAME}
)

corrosion_install(
  TARGETS ${PROJECT_NAME}
  RUNTIME DESTINATION ${CATKIN_PACKAGE_BIN_DESTINATION}
)
