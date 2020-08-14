# Assimilate

A rust binary and shell function to make exporting/aliasing less painful

## Motivation

How often do you need to export something or make an alias and decide you really want that globally available? If you are like me, you do this quite often, but are just too lazy to add it to your alias or export file _after_ you have it working in your shell. __With _Assimilate_ you can alias/export to your current shell and/or directly to your alias/export file.__

## Usage

Assimilate is comprised of two parts, a `rust` binary and a `shell function`. The `rust` binary handles all the heavy lifting and provides the nice `cli` interface, while the `shell function` enables you to alias/export to the current shell. You can use _just_ the `rust` binary, but you will not be able to alias/export to the current shell.

```shell
$> assimilate_bin --help
> USAGE:
      assimilate_bin [FLAGS] --name <name> [item]...
  
  FLAGS:
      -a, --alias
      -e, --export
          --help       Prints help information
      -h, --here
      -s, --save
      -V, --version    Prints version information
  
  OPTIONS:
      -n, --name <name>
  
  ARGS:
      <item>...
```

__The default behavior is to _export_ and you must provide either `-h` or `-s` for any action to be taken.__

Additionally, if you want to `--save` to your alias/export file, you will need to have already __exported__ one or both of the following ...

```shell
EXPORT_FILE=<path to export file>
ALIAS_FILE=<path to alias file>
```

## Examples

```shell
$> assimilate -ahn foo echo bar
# --> result: alias foo='echo bar'

$> assimilate -ehn MY_OTHER_HOME '$HOME/subdir'
# --> result: export MY_OTHER_HOME='$HOME/subdir'

$> assimilate -ehn EXPANDED_HOME $HOME/subdir
# --> result: export EXPANDED_HOME='<path_to_your_$HOME>/subdir'
# see discussion subsection Shell Expansion

$> dig +short myip.opendns.com @resolver1.opendns.com -4
# <your ip address>
$> assimilate -asn myip "!!"
# alternatively: assimilate -asn myip -- !!
# see discussion subsection Shell Expansion
# --> result alias myip='dig +short myip.opendns.com @resolver1.opendns.com -4'
```

## Caveats

Because you are specifying arguments on the command line you will want to ___pay attention to shell expansion___.

_Be aware_ that the `shell function` utilizes __`eval`__ in order to alias/export in the current shell.

## Installation

Currently you must clone and build from source ...

```shell
$> git clone https://github.com/fvhockney/assimilate.git
$> cd assimilate
# Make sure you have a rust toolchain installed
$> cargo build --release
# make sure that target/release/assimilate_bin is in your $PATH
```

if you want to use the shell function ...
```
--- in someplace which will run on shell startup
source <path to assimilate git dir>
```

alternatively if you want to just use the rust binary
```shell
$> ln -s assimilate_bin /usr/sbin/assimilate
```


## Discussion

### Why a binary and a function

This boils down to three issues:

First is __Process context__. If you want to alias/export in the current shell, you _must_ perform that opertion within the context of that shell. Running the command in a seperate binary causes a subprocess which can not affect the parent/starting process. However, executing `alias`/`export` from within a `shell function` _is_ successful because it is running in the same process context.

Second is __Process pollution__.  Writing everything within the `shell function` is certainly feasible (in fact, I did just this in one iteration), but this leads very easily to polluting the running process with lots of variables/functions. Sure, I could probably get around the function problem by using declarative programming, but functions are just _sooooo_ much better. The variable problem is much more insidious, you need variables for parsing the command line arguments plus quite a number of holder variables. Cleaning them up with __unset__ is generally possible, but since you are in a function, you can't really utilize traps to capture errors and ensure clean up. Additionally, unsetting varialbes from a sourced function is tricky as they may not reliably be re-instantiated when the function is called again. Putting the bulk of the work load in a seperate binary prevents a great deal of process pollution.

Third is __Modularity__. Maybe you don't care about setting it in the current process. Fine, you can just use the binary and not worry about sourcing the `shell function`.

Third the second, __Portablity__. The shell function is, to the best of my abilities, `POSIX` compliant. Writing the whole shebang (pun intended) in a `POSIX` manner without requiring the user to install any dependencies (ok, fine, most people probably have `sed`, `tr`, `cut` and the like already installed), is quite the challenge. Plus, I would really like to expand this to allow history navigation and other functionality which would have just been a nightmare in pure `shell`. Git* et al pipelines coupled with releases and packaging files makes it easy for users to install and use, even without having to install the `rust` toolchain.

### Why `eval`

I really don't like `eval`. It's kind of like seeing `unsafe`. Even if it is _safe_, something just turns in my stomach, but alas, because of items 1 and 2, I could find no other way of handling this problem. Essentially, if the `rust` binary exits with `0`, then the shell function `eval`s the result. If it exits with anything else, it `echo`s the result and `returns` with the original exit code. This means we only have to save two variables in the current scope (`__assimilate_exit_code` and `__assimilate_output`).

### Shell Expansion ...

will get you every time. Know how shell expansion works on your shell and don't let it bite you in the rear. The `rust` binary will _always_ `'` the right hand side of the expression so that the `eval` statement does not expand if you did not want it to.

## TODO

- [ ] interactive history lookup
- [ ] `parse` option to handle commands like `export foo='bar'; assimilate -se "!!"`
- [ ] tests of binary
- [ ] package (if there is enough interest)
- [ ] CI/CD with releases
- [ ] better help documentation
- [ ] man pages?
