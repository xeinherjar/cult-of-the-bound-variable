# The Universal Machine

Specifications from [Cult of the Bound Variable](http://www.boundvariable.org/task.shtml)
You can read their [paper](http://www.boundvariable.org/press/tr-06-163.pdf).


This is a VM for the Universal Machine, a programming challenge from the 2006
ICFP Programming Contest, built to help me play with a few languages.
So far its working for Python, Rust and Javascript.

After passing the initial test it is clear that this implementation is slow.
Part of that is Python and I'm sure the other part is me.

Reading [online](http://www.cs.tufts.edu/comp/40/um/) it appears a C version
can complete the sandmark run in one minute.

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
