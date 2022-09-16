# RFC: config file

## Overview

It would be good to have a single opinionated config file
that allows you to configure the following parameters (for now):

- PKI directory
- domain name (suffix?)
- verbosity / logging level
- socket name (?)

I suggest using:

- default locations (in order of override):
  - /etc/aurae/settings.toml
  - ~/.aurae/settings.toml
  - --config file to CLI tooling

The config file is read and managed by the `config` crate.
