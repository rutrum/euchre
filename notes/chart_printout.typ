#set page(flipped: true)

#let data = json("../py_euch/data/32players_15rounds.json")
#let chart = data.chart

#show table.cell.where(body: [5]): strong
#set table(
    fill: (x, y) => if (y < 2 and x > 1) 
        or (x < 2 and y > 1) { gray.lighten(50%) },
    stroke: (x, y) => if x < 2 and y < 2 {
        none
    } else {black},
)

#table(
    columns: data.num_players + 2,
    rows: data.num_rounds,
    align: left,
    inset: 0.5em,
    table.cell(
        colspan: 2, 
        rowspan: 2, 
        []
    ),
    table.cell(
        align: center,
        colspan: data.num_players,
        [Seat]
    ),
    ..for player in range(data.num_players) {(
        [#(player+1)],
    )},
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