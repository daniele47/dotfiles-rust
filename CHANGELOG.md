# Changelog

All notable changes to this project will be documented here

TODO:
- [ ] save/restore should allow also deleting files from the invalid side, with a --dangerous flag for restore to allow deleting files from home!
- [ ] add logging to files in `$AUTOSAVER_LOGFILE` / `--logfile`, so that i get automatic logs on file
- [ ] more powerful prompts (maybe with this i could remove the -d/-s/... flag? or maybe keep both, maybe both be useful!):
    - [ ] `N` to skip entire profile
    - [ ] `d` to show diff
    - [ ] `s` to show file/script in its entirety
    - [ ] `e` to edit the file?
    - [ ] `d` to delete the file?
- [ ] `all` profile could be reserved for all profiles?
- [ ] `default` profile could be reserved to specify what to run when no profiles are specified otherwise? this would be fallback if `--profile` flag is not passed

## v2.0.0

### Features

- complete rewrite, using 3rd party crates as dependencies
- added `--log` flag with proper logs

## v0.18.1 / v1.0.0

### Changes

- multithreaded stdout/stderr handler, which nicer split between the two visually

## v0.18.0

### Features

- added `--show-types|-t` for `list|save|restore` commands

## v0.17.5

### Changes

- brough back support for symlinks in dir (just with check they don't escape containment)

## v0.17.4

### Patches

- fixed inout to make sure there are no broken colors

## v0.17.3

### Patches

- cut output now stays precisely in 80 char width

## v0.17.2

### Patches

- `clear` now checks ALL, to allow directories shared between profiles

## v0.17.1

### Patches

- disabled hidden config files and directories, as those make possible to override . and "" (bad!)!

## v0.17.0

### Features

- added `--add|-a` to `clear` command, to allow deleting untracked files outside profile dirs

## v0.16.1

### Patches

- fixed debug msg for symlink checks

## v0.16.0

### Features

- added `dir` option type in `module|runner` profiles to indicate the dirname (allows easy config refactor)
- added `--show-dir|-d` flag to show the directory in `tree` command

## v0.15.1

### Patches

- deleting symlinks now delete all parent dirs too

## v0.15.0

### Features

- added `--unique|-u` to `tree` command to skip already seen profiles

## v0.14.4

### Patches

- fixed wrongly parsed single letter flag 

## v0.14.3

### Patches

- made backtrace capture optional, as it significantly slows down the program on failure

## v0.14.2

### Patches

- errors and warnings now don't color the entire line anymore, just the `ERROR|WARNING:` part 

## v0.14.1

### Patches

- do not panic on broken stdout/stderr. Just keep running silently

## v0.14.0

### Features

- added single letter flag shortscuts to tree word flags
- added `--ascii|-a` flag for `tree` command to use only ascii characters

### Patches

- added checks for profile used, to avoid parent dirs in it and to avoid it being an absolute path
- improved err msg for commands with invalid args

## v0.13.0

### Features

- added `tree` command to display the resolution tree of profiles
- added `--short-names` flag for `tree` command to show only basename of profiles
- added `--show-types` flag for `tree` command to show the type of the profiles

### Patches

- stricter `Runner` methods, now don't borrow mutably anymore

## v0.12.1

### Changes

- file diffs now separate different diff blocks with a nice `@` sign

## v0.12.0

### Features

- added `backtrace` for errors
- adding debug options to `inout`
- added `--debug` flag to show debug output
- added more logs for `--debug`

## v0.11.0

### Features

- `--symlinks|-s` in `clear` to handle broken symlinks too
- `--unmodified|-u` flag in `list` command to show also tracked but not modified files

### Patches

- fixed broken symlinks causing an error. Now they are handled properly
- fixed `list` command not showing deleted files

## v0.10.1

### Changes

- now `clear` command accepts a profile, and only clears relative to that profile

## v0.10.0

### Features

- added `clear` command to remove all untracked files from `run` and `backup` dirs

### Changes

- added back `list` as semplified alias for `save -l`

## v0.9.1

### Fixes

- use 80 as line len everywhere

## v0.9.0

### Features

- `--full|-f` flag to show entire diff, script and script output

## v0.8.5

### Changes

- changed `list` command with `--list|-l` flag

## v0.8.4

### Changes

- allow missing `run`, `backup`, `config` directories
- made flags consistent, by actually splitting words with a `-` 

## v0.8.3

### Patches

- symlinks check doesn't run on help/version actions

## v0.8.2

### Changes

- added `$` to start of prompt lines

## v0.8.1

### Patches

- updated colors for rmhome and rmbackup paths
- parser does not allow .. paths anymore
- binary now checks no symlink that links to outside the repo exist before running 

## v0.8.0

### Features

- config directories are now treated exactly as if they were composite profiles loading the files within

## v0.7.4

### Changes

- added `ls` alias for `list`

## v0.7.3

- added strict command checks

## v0.7.2

### Patches

- fixed `autosaver` script deleting old version even if newer fails to install
- actually parse stdout and format it nicely

## v0.7.1

### Fixes

- proper color for line separator between scripts

## v0.7.0

### Features

- bash `autosaver` script allows specifying the precise version to download

## v0.6.4

- run command now isolates scripts based on the profile

## v0.6.3

### Changes

- `--dryrun` flag turned into `-l|--list` flags in `run` command

## v0.6.2

### Patches

- script output now isn't parsed, and kept as is
- script output now it's ended by a clear line separator
- added output showing the main profile before all else

## v0.6.1

### Patches

- add `.default` file to help msg
- fixed global help msg invalid line

## v0.6.0

### Features

- added `.default` configuration file to specify a default profile

### Patches

- updated help msg to explain global flag better
- make scripts automatically executable before running them

## v0.5.1

### Patches

- fixed error printing one extra whiteline
- better err msg for when no profile is specified
- captured scripts output and formatted nicely
- very simple multithreaded stdout/stderr

## v0.5.0

### Features

- added new runner profile
- added new run command

### Patches

- removed valid name checker for composite profiles
- fixed `-n|--assumeno` not actually properly working
- fixed `-n|--assumeno` and `-y|--assumeyes` output, now they print y|n properly

## v0.4.2

### Patches

- updated bash script to shorten its runtime, and get a faster running cli
- fixed `--assumeyes|-y` and `--assumeno|-y` flags, now properly skipping prompt

## v0.4.1

### Patches

- flags with no commands now are treated as errors too.
- better profile output color
- binary files are now properly handled by the program when diffing
- fixed `content_eq` function, and now is able to compare binary files

## v0.4.0

### Features

- implemented Myers algorithm to diff files, and used in new `-d|--diff` flag in `save|restore|list`
- added q into prompt to instantly quit

### Patches

- improved cli colors

## v0.3.5

### Removals

- removed autocompletions, and self-updating bash script

## v0.3.4

### Patches

- install script now shows downloaded version

## v0.3.3

### Patches

- changing bash script to have almost no logic, and leave all the logic in update scripts in the repo itself
- update help msg to list environment variables

## v0.3.2

### Patches

- fixes `-a` not working with list command

## v0.3.1

### Patches

- fixed `rmhome|rmbackup` not showing the paths

## v0.3.0

### Features

- added `AUTOSAVER_HOME` and `AUTOSAVER_PROFILE` configuration env variables
- added `rmhome` and `rmbackup` actions, to delete files from home/backup directories
- implemented `--help` to get help messages from current available commands

### Patches

- always show relative paths instead of home paths (which was very arbitrary and useless)

## v0.2.0

### Features

- added bash script to automatically download latest autosaver binary

## v0.1.0

### Features

- added `list`, `save`, `restore` commands to list differences between home and backup, and save/restore them
- `--version` to get the binary current version
