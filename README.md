Programming Cheatsheet
---
saves to ~/.config/cheatsheet.yaml and online to a GitHub gist
=> synchronize between local and cloud during commands

`show` # shows table & syncs if GitHub pat and gist_id are present in config

`add <name> <value>` # adds a record with a name and a value

`remove <record id>` # removes record w/ specified id from table and cloud. ID is automatically assigned to record when the record is added to table.

`web-init` # prompts for pat -> creates gist + saves gist id in config

`get` # prompt for id -> copies value w/ corresponding id into clipboard
