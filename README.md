# ets
`ets`: Ensure the Same

### What is ets?

`ets` (Ensure the Same) is a program similar to the Unix utility [TripWire](https://en.wikipedia.org/wiki/Open_Source_Tripwire) ([src](https://github.com/Tripwire/tripwire-open-source)) a host-based intrusion detection system. You provide `ets` with a top-level directory, and it recurses through computing the hash of each file. These hashes are then compared to a previous run, and all files that have changed are reported. There are numerous configuration options to exclude files, ensure files exist, reporting options, etc.

Most-of-the-time you'd run `ets` on a single machine and use it as a way to see if files have changed between runs. `ets` is not initially designed with security in mind, and is more used as a configuration management tool. Changing a file, and the corresponding hash of that file would not be hard for an attacker.

### Installing

### Configuration

`ets` is configured primarily via a configuration file that is expressed in YAML. Below are the following configuration options specified in the configuration file.

#### Configuration File

`root_dir` - (REQUIRED) This is the top-level directory (or directories) that are to be searched during the run. This is typically `/etc` as that is where most configuration files are found on a Linux machine.

`data_file` - (REQUIRED) The data file where `ets` saves the data after an `--update` run, or reads the data to compare. What is saved in this file is the list of files and their corresponding hashes. If you want to create a "golden image" and distribute it to many machines, then copy this file to all of those machines.

`report_dir` - (REQUIRED) The directory where reports are saved after each run. Reports are in JSON (compromise between human readable and machine readable), and report all the files that were found to not match between runs.

`report_proc` - (OPTIONAL) The process to run after `ets` has finished and generated a report. `ets` will run the program, and pass as the only command line argument the full path to the report file it generated.

`exclude` - (OPTIONAL) The list of files (full path or basic [glob-style regex](https://en.wikipedia.org/wiki/Glob_(programming))) that should be excluded during the run. You can also specify specific line numbers inside the file by appending a colon and the line numbers separated by commas or ranges with dashes. For example, to exclude lines 5, 7, and the range 13 through 17 in the `my.cnf` file: `my.cnf:5,7,13-17`

#### Command Line Options

`--update` - This flag will update the database stored in the `data_dir` in the configuration file. Any existing data will be truncated/removed. This is how you generate the initial database of files and hashes.

`--config` - Specifies the config file to read from. If this command line argument is not specified, then the following order will be searched: current directory, `/etc/ets.config`. If not configuration file is found, an error is printed to STDERR and the program will exit without doing anything.
