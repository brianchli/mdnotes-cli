# mdnotes-cli

mdnotes-cli is a markdown based note taking utility that is built based on my workflow, with extensions and features based on what piques my curiosity at the current moment.

It is built with the intention to be used with other terminal commands and utilities such as [grep](https://www.gnu.org/savannah-checkouts/gnu/grep/manual/grep.html), [sed](https://www.gnu.org/software/sed/manual/sed.html) and [fzf](https://github.com/junegunn/fzf).

## Usage
**mdnotes-cli** is invoked by invoking `notes` followed by a subcommand. The current available subcommands are:
*  `create` - create a new note
*  `list` - list notes in the workspace
*  `config` - configuration options
*  `save` - save functionality that is a wrapper around git
*  `switch` - move to a different context ('notebook') for note storage
*  `notebook` - conduct operations on 'notebooks'
    * create - create a new notebook
    * remove - remove an existing notebook

## Roadmap
Below is a list of features that I'm currently interested in implementing at some point:
* [ ] Transition command line parsing from the [clap](https://github.com/clap-rs/clap) library to the [argh](https://github.com/google/argh) library
* [x] Implementation of a multi ~~workspace~~ **notebook** system
* [ ] Implementation of Daemon layer for note tagging and caching
* [ ] Custom error message implementation; potentially with [anyhow](https://docs.rs/anyhow/latest/anyhow/)
