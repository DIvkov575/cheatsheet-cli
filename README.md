Programming Cheatsheet
---
saves to ~/.config/cheatsheet.yaml and online to a github gist
=> synchronize between local and cloud during commands

`show` # shows table & syncs if GitHub pat and gist_id are present in config

`add <name> <value>` # adds a record with a name and a value

`remove <record id>` # removes record w/ specified id from table and cloud. Id is automatically assigned to reccord when the reccord is addded to table.

`web-init` # prompts for PAT -> creates gist + saves gist id in config