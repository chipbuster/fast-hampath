criterion bench compare is a little rough around the edges (e.g. args are wrong)

bench compare action does not automatically pull master if there are updates. If master
is updated in the meantime, it will not build something new.

Old files are retained

Depends on master branch: new benchmarks on head cnanot run on master