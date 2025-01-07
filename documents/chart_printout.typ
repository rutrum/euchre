#set page(flipped: true)

#let data = json("../py_euch/data/chart.json")
#let chart = data.chart

#let num_tables = calc.floor(data.num_players / 4)

// #show table.cell.where(body: [5]): strong
#set table(
    fill: (x, y) => if (y < 1 and x > 1) 
        or (x < 2 and y > 0) { gray.lighten(50%) },
    stroke: (x, y) => if x < 1 and y == 0 {
        none
    //} else if calc.rem(x + 2, 4) == 2 and x > 1 {
    //    (
    //        top: 0.5pt,
    //        bottom: 0.5pt, 
    //        left: 1pt,
    //        right: 0.5pt,
    //    )
    } else {
        0.5pt + black
    },
)

// todo: vertical lines between partners

#table(
    columns: data.num_players + 2,
    align: left,
    inset: 0.4em,
    column-gutter: (
        (auto, )
        + (6pt, auto, 6pt, auto) * num_tables
    ),

    table.cell(
        colspan: 2, 
        []
    ),

    // row headers
    ..for (i, euchre_table) in range(num_tables).enumerate() {(
        table.cell(
            align: center,
            colspan: 4,
            [Table #(i+1)]
        ),
    )},
    //..for player in range(data.num_players) {(
    //    [#(player+1)],
    //)},

    table.cell(
        align: horizon,
        rowspan: data.num_rounds,
        rotate(-90deg, [Round])
    ),
    ..for (i, row) in data.chart.enumerate() {(
        [#(i+1)],
        ..for player in row {(
            [#(player+1)],
        )}
    )}
)