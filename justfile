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
