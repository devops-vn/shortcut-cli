## shortcut-cli

shortcut-cli is a simple shortcut cli to create a script as a short macro command in linux and darwin.

### How to install it or compile it from source?

  - For linux kernel: simply just download prebuilt binary (x86_64).
  - For darwin kernel (macOS): download this package installer and run it.
  - Clone latest source from this repository and build it by manually from your machine.

Download binary file for linux kernel:
```sh
$ curl -s https://github.com/cuonglb/shortcut-cli/releases/download/v1.0.0/shortcut-cli -o /usr/local/bin/shortcut-cli
$ chmod +x /usr/local/bin/shortcut-cli
```

Download package installer for darwin kernel (macOS):
```sh
$ wget https://github.com/cuonglb/shortcut-cli/releases/download/v1.0.0/shortcut-cli-macos-installer-x64-1.0.0.pkg
```

Build from source:

> Install rust: https://www.rust-lang.org/tools/install

> Make sure your are on rust toolchain latest stable.

> Install GNU make.

```sh
$ cd shortcut-cli
$ make
```

### How to use shortcut-cli?
You can see list of subcommands:
```sh
$ shortcut-cli macro

USAGE:
    shortcut-cli macro <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    create       Create new macro script.
    delete       Delete macro(s) script.
    describe     View content of macro(s) script.
    help         Prints this message or the help of the given subcommand(s)
    hub          Management existing macros from shortcut-ot.net hub
    ls           Listing all macro scripts in this marchine.
    make-bash    Make bash script for macro(s) in this marchine (Require sudo privileges) e.g.: /usr/bin/foo ;
                 /usr/bin/bar
    run          Run macro(s).
```

```sh
$ shortcut-cli macro hub

USAGE:
    shortcut-cli macro hub <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add       Add existing macro to local machine.
    help      Prints this message or the help of the given subcommand(s)
    search    Search macro(s) on a macro hub. e.g.: pastebin.com
```

### Examples:

create a short macro command
```sh
$ shortcut-cli macro create --content 'git pull origin master' --key gpom
Oct 29 19:16:17.004 INFO macro create
Oct 29 19:16:17.004 INFO save-or-update key=gpom value=git pull origin master
Oct 29 19:16:17.004 INFO success
```

delete a short macro command (requires sudo if it get `make-bash` generated)
```sh
$ sudo shortcut-cli macro delete --key gpom
Oct 30 13:13:42.435 INFO macro delete
Oct 30 13:13:42.435 INFO delete macro key=gpom
Oct 30 13:13:42.437 INFO success
```

listing all short macro commands
```sh
$ shortcut-cli macro ls
Oct 29 19:18:11.769 INFO macro listing
KEY                  CONTENT
gpom                 git pull origin master
```

make a bash script to run the short macro command (requires sudo)
```sh
$ sudo shortcut-cli macro make-bash --key gpom
Oct 29 19:20:06.670 INFO macro make bash
Oct 29 19:20:06.671 INFO make bash script for macro key=gpom
```

now you can just run gpom command from a git repository :)
```bash
$ gpom
From github.com:foo/bar
 * branch              master     -> FETCH_HEAD
Already up to date.
```

create a ton of short macro commands and share them with your friends :), i am using `pastebin.com` for to do that.

```sh
$ curl -s https://pastebin.com/raw/JsqSJABQ
docker-cleanup:
  content: |
    docker stop $(docker ps -a -q)
    docker rm $(docker ps -a -q)
aptu:
  content: sudo -- sh -c 'apt update; apt upgrade -y'
gpom:
  content: git pull origin master
df-G:
  content: df -BG
```

search a short macro command from a macro hub like `https://pastebin.com/raw/JsqSJABQ`
```sh
$ shortcut-cli macro hub search --key "*"
Oct 30 13:56:04.314 INFO updating macro list from hub url: `https://pastebin.com/raw/JsqSJABQ/Eqi7uExx`
Oct 30 13:56:04.814 INFO updated success
Oct 30 13:56:04.817 INFO found macro key=aptu
CONTENT
sudo -- sh -c 'apt update; apt upgrade -y'
Oct 30 13:56:04.817 INFO found macro key=gpom
CONTENT
git pull origin master
Oct 30 13:56:04.817 INFO found macro key=docker-cleanup
CONTENT
docker stop $(docker ps -a -q)
docker rm $(docker ps -a -q)

Oct 30 13:56:04.817 INFO found macro key=df-G
CONTENT
df -BG
```
```sh
$ shortcut-cli macro hub search --key "docker*"
Oct 30 13:56:35.192 INFO updating macro list from hub url: `https://pastebin.com/raw/JsqSJABQ/Eqi7uExx`
Oct 30 13:56:35.663 INFO updated success
Oct 30 13:56:35.666 INFO found macro key=docker-cleanup
CONTENT
docker stop $(docker ps -a -q)
docker rm $(docker ps -a -q)
```

add a short macro command from a macro hub to local machine
```sh
$ shortcut-cli macro hub add --key docker-cleanup
Oct 30 13:58:40.854 INFO updating macro list from hub url: `https://pastebin.com/raw/JsqSJABQ/Eqi7uExx`
Oct 30 13:58:41.327 INFO updated success
Oct 30 13:58:41.330 INFO found macro key=docker-cleanup
Oct 30 13:58:41.330 INFO add macro key=docker-cleanup to local machine
Oct 30 13:58:41.331 INFO save-or-update key=docker-cleanup value=docker stop $(docker ps -a -q)
docker rm $(docker ps -a -q)

Oct 30 13:58:41.331 INFO success
```

### How to the shortcut-cli config?

The shortcut-cli yaml config is located at `$HOME/.shortcut-cli/config.yml`

There is an `config.yml.example` in the repository, lets try it
```sh
$ mkdir -p ~/.shortcut-cli
$ cat <<EOF > ~/.shortcut-cli/config.yml
hub_url: https://pastebin.com/raw/JsqSJABQ
EOF
```

### Config parameters

shortcut-cli is using YAML format for config.

| Param | Default value |
| ------ | ------ |
| bash_header | `"#!/bin/bash -e"` |
| bin_dir | /usr/local/bin |
| log_type | term |
| hub_url | https://pastebin.com/raw/Eqi7uExx |

##### Todos

 - I haven't todos yet

### License
BSD
