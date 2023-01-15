# OTP Starter

A simple CLI application that is used to run the OpenTripPlanner in 
an production environment without the required overhead of handling process IDs and struggling arround to run 
it as a deamon.

## How it works

The application uses the unmodified releases of the OTP. It is just ab abstraction layer onto the CLI of the executable 
.jar file that is used to run the server and perform all the necessary operations.
It starts the application using [nohup](https://de.wikipedia.org/wiki/Nohup) to run it as a deamon. 
It is stopped by just fetching the process ID of the running server and terminating the JVM that runs
the server. 

## System requirements

This application does not work in windows.

You will need: 
- Java 17
- Linux/UNIX based operating system

## How to use it

First you will have to download the [latest release]() from the releases tab.

After that you can add it to your `/usr/local/bin` folder or leave it in the directory.
If you decide to leave it in the directory you downloaded it into, you will be forced everytime to 
provide the full path to the executable if you want to run it.

After that go into the folder you want to use as your folder for the OTP installation

This folder should contain:
- A supported version of OTP [2.1.0, 2.2.0]
- A directory that contains all your data like GTFS files, etc.

Now create a file named `otp-starter.toml` in this directory and use this configuration schema:

```toml
# The amount of memory that is assigned to the JVM that runs the OTP
memory_limit = "10G"

[http]
# The HTTP port that uses the application
port = 8081
# The HTTPS port that uses the application
secure_port = 8083

[path]
# The relative/absolute path to the data directory
data_dir = "./data"
# The relative/absolute path to the directory that contains
# the graphs.
graph_dir = "./graphs"
```

Now you are ready to launch your application.

If you want to start it directly and build the graph on startup just run this command:

```shell
otp-starter start -bos
```

If you prefer to build a graph before use these commands:

```shell
otp-starter buildGraph && otp-starter start
```


If you want to stop all running OTP instances just use 
```shell
otp-starter stop
```

