# Corrosion Catkin Tests

Set up are the follosing catkin packages:
- `cdylib` (c ffi rust dynamic library)
- `dylib` (rust dynamic library)
- `rlib` (rust static library)
- `staticlib` (c ffi rust static library)
- `bin` (rust package trying to use the above)

A Devcontainer setup for VSCode is provided which sets up ROS, catkin_tools and a user account with a nightly rust installation.

To try to compile it, just run `catkin build`. Currently nothing works, c ffi libraries (`cdylib` and `staticlib`) are not installed by corrosion and rust libraries (`dylib` and `rlib`) are rejected by corrosion as not being a valid target.
