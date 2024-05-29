Programming Cheatsheet
---
saves to ~/.config/cheatsheet.yaml and online to a GitHub gist
=> synchronize between local and cloud during commands

`show` # shows table & syncs if GitHub pat and gist_id are present in config

`add <name> <value>` # adds a record with a name and a value

`remove Vec<record id>` # removes records w/ specified ids from table and cloud. provide argument as list of strings

`web-init` # prompts for pat -> creates gist + saves gist id in config

`get` # prompt for id -> copies value w/ corresponding id into clipboard
