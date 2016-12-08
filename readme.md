# The Universal Machine

Specifications from [Cult of the Bound Variable](http://www.boundvariable.org/task.shtml),
[paper](http://www.boundvariable.org/press/tr-06-163.pdf).


This is a VM for the Universal Machine, a programming challenge from the 2006
ICFP Programming Contest, built to help me play with a few languages.
So far its working for Python, Rust and Javascript.

## Sandmark Benchmark
#### MacBook Pro 13, Late 2013 (11,1) macOS sierra 10.12.1
```
JavaScript (FireFox): 40 minutes
JavaScript (Chrome): 40 minutes
Python:
PyPy: 6.5 minutes
Rust: 35 seconds
```

## Running the VM in Python
You can run the test file, sandmark.umz or codex.umz with:
`pypy universal-machine.py sandmark.umz`

When you start the codex it will prompt for a code, enter:
`(\b.bb)(\v.vv)06FHPVboundvarHRAk`

It will then prompt to (x)exit or (p)dump UM data.


If you dump, it just spits it out to stdout.
```pypy universal-machine.py codex.umz |& tee umdata.um```
I've tried from both Python and Rust, but the file generated has some text at
the top that says `UM program follows colon:` but then its random jumk.


My assumption is that you run it back though the UM, but so far it fails to
load.  So possibly I'm outputing wrong, or something else is wrong.
