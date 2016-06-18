# The Universal Machine

Specifications from [Cult of the Bound Variable](http://www.boundvariable.org/task.shtml)

This is a VM for the Universal Machine, a programming challenge from the 2006
ICFP Programming Contest, built to help me learn some Python.

After passing the initial test it is clear that this implementation is slow.
Part of that is Python and I'm sure the other part is me.

I made some quick changes to have it run under PyPy, but it still isn't very
fast.

Reading [online](http://www.cs.tufts.edu/comp/40/um/) it appears a C version 
can complete the sandmark run in one to two minutes.

Maybe I should try this in Rust, C or Scala?

## Running the VM
You can run the test file, sandmark.umz or codex.umz with: 
`pypy main.py sandmark.umz` or `pypy main.py codex.umz`
If you wait long enough codex.umz will prompt for a code...
