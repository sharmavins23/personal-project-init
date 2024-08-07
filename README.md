# Personal Project Initializer

Every time I make a new project I have to go through the following steps:

```
$ git init
$ touch README.md
  # Add in README name and MIT License TL;DR
$ touch LICENSE.md
  # Add in license text
```

This project automates that process to one singular command, `ppi`, in the
terminal.

Within the `/bin/` folder (which is cloaked from git) I have the following batch
file:

```batch
@echo off
python <path>personal-project-init\scripts\ppi.py
```

This `/bin/` folder is added to my PATH environment variable.

# License TL;DR

This project is distributed under the MIT license. This is a paraphrasing of a
[short summary](https://tldrlegal.com/license/mit-license).

This license is a short, permissive software license. Basically, you can do
whatever you want with this software, as long as you include the original
copyright and license notice in any copy of this software/source.

## What you CAN do:

- You may commercially use this project in any way, and profit off it or the
  code included in any way;
- You may modify or make changes to this project in any way;
- You may distribute this project, the compiled code, or its source in any way;
- You may incorporate this work into something that has a more restrictive
  license in any way;
- And you may use the work for private use.

## What you CANNOT do:

- You may not hold me (the author) liable for anything that happens to this code
  as well as anything that this code accomplishes. The work is provided as-is.

## What you MUST do:

- You must include the copyright notice in all copies or substantial uses of the
  work;
- You must include the license notice in all copies or substantial uses of the
  work.

If you're feeling generous, give credit to me somewhere in your projects.
