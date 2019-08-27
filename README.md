r2r-echo - Example use of the r2r crate.
=============
Simple cli app that demonstrate the <https://github.com/sequenceplanner/r2r> crate. Basically ros2 topic echo with a curses frontend (using <https://github.com/fdehau/tui-rs>).

How to run
---------
Clone r2r into a neighboring directory in order to be able to tell it which messages types to compile.

```
> git clone https://github.com/sequenceplanner/r2r.git
> git clone https://github.com/sequenceplanner/r2r-echo.git
> ros2 msg list > r2r/msgs.txt
> cd r2r-echo
> cargo run --release /topic_name
```

Press 'q' or escape to quit.
