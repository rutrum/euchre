default:
    just watch documents/chart_printout.typ

watch FILE:
    typst watch --root . {{FILE}}

open FILE:
    zathura {{FILE}} &
