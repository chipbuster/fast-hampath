criterion bench compare is a little rough around the edges (e.g. args are wrong)

bench compare action does not automatically pull master if there are updates. If master
is updated in the meantime, it will not build something new.

behavior can be confusing: both master and test need to have correct benches,
but only test needs correct yml (I think??)

Old files are retained (attempt to use clean to solve this?)

Depends on master branch: new benchmarks on head cnanot run on master