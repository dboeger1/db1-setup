# db1-setup

db1-setup makes it trivial to set up a productive development environment across
several target platforms. It bundles a collection of configuration files with a
CLI utility for installing them, as well as other useful software.

Features:
- Install Neovim configuration.
- Install tmux configuration.
- Install useful packages for software development.
- Automate initialization of Rust using rustup.
- Automate initialization of Incus for hosting development containers.
- Automate initialization of Podman for hosting application containers.

To be clear, this project is focused primarily on the needs of GitHub user
@dboeger1. That being said, the project is simple to modify, and could
definitely serve as a starting point for other software developers.


Supported Platforms
-------------------

The goal is to support all the following targets, but here is their current
working status:

|             | Linux  | MacOS | Windows |
|:-----------:|:------:|:-----:|:-------:|
| **x86_64**  | Fedora | No    | No      |
| **aarch64** | Fedora | No    | No      |

Note that this project is still in early development. In particular, I'm still
figuring out how best to handle different Linux distributions, as they often
have different conventions for installing software.


Packages
--------

This project will eventually produce and publish platform-native packages for
all targets. It's currently limited to building an RPM on Fedora, which I am not
yet publishing, as I need to stabilize the project a bit more.
