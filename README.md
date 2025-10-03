# mdnotes-cli

mdnotes-cli is a markdown based note taking app that is built entirely for my own workflows, with extensions and features based on what piques my curiosity at the current moment.

It is built with the intention to be used with other terminal commands and utilities such as [grep](https://www.gnu.org/savannah-checkouts/gnu/grep/manual/grep.html), [sed](https://www.gnu.org/software/sed/manual/sed.html) and [fzf](https://github.com/junegunn/fzf).

## Usage
**mdnotes-cli** is invoked by invoking `notes` followed by a subcommand. The current available subcommands are:
*  `create` - create a new note
*  `list` - list notes in the workspace
*  `config` - configuration options
*  `save` - save functionality that is a wrapper around git

## Roadmap
Below is a list of features that I'm currently interested in implementing at some point:
* [ ] Transition command line parsing from the [clap](https://github.com/clap-rs/clap) library to the [argh](https://github.com/adishavit/argh) library
* [ ] Implementation of a multi workspace system

