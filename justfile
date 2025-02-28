all:
    #!/usr/bin/env bash

    for i in $(seq 24 4 48); do
        (cd chart_gen; uv run gen.py --players $i)
        typst compile --root . --input players=$i documents/chart_printout.typ documents/chart_printout_$i.pdf
    done

default:
    just watch documents/chart_printout.typ

compile FILE:
    typst compile --root . {{FILE}}

watch FILE:
    typst watch --root . {{FILE}}

open FILE:
    zathura {{FILE}} &

code:
    codium .
