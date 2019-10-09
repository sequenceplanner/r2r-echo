r2r-echo - Example use of the r2r crate.
=============
Simple cli app that demonstrate the <https://github.com/sequenceplanner/r2r> crate. Basically ros2 topic echo with a curses frontend (using <https://github.com/fdehau/tui-rs>).

How to run
---------
1. Source your ROS workspace.
2. Then either clone the repo and run:
```
> git clone https://github.com/sequenceplanner/r2r-echo.git
> cd r2r-echo
> cargo run --release /topic_name
```
or use cargo to intall the binaries on your system.
```
> cargo install --git https://github.com/sequenceplanner/r2r-echo
> r2r-echo /topic_name
```

Press 'q' or escape to quit.

3. If your ROS messages have changed either rebuild it if you cloned the repo, or run cargo install again with --force to force a refresh.
