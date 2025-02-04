#set page(flipped: true)

#let data = json("../py_euch/data/chart.json")
#let chart = data.chart

#let max_tables_per_row = 7;
#let num_tables = calc.floor(data.num_players / 4)
#let num_rows = calc.floor((num_tables - 1) / max_tables_per_row) + 1

#set page(margin: 0.5in, paper: "us-letter")


#show heading.where(depth: 1): set align(center)
= #data.num_players Person Rotation Chart
#v(1em)

// #show table.cell.where(body: [5]): strong
#set table(
  fill: (x, y) => if (y < 1 and x > 1) or (x < 2 and y > 0) { gray.lighten(50%) },
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

#{
  for row in range(num_rows) {
    let start_table = row * max_tables_per_row;
    let row_tables = if row < num_rows - 1 {
      max_tables_per_row
    } else {
      num_tables - row * max_tables_per_row
    };

    // [#row #start_table #num_tables #row_tables]

    table(
      columns: row_tables * 4 + 2,
      align: left,
      inset: 0.4em,
      column-gutter: (
        (auto,) + (10pt, auto, 6pt, auto) * row_tables
      ),

      table.cell(
        colspan: 2,
        [],
      ),

      // row headers
      ..for (i, euchre_table) in range(start_table, start_table + row_tables).enumerate() {
        (
          table.cell(
            align: center,
            colspan: 4,
            [Table #(i + start_table +1)],
          ),
        )
      },
      //..for player in range(data.num_players) {(
      //    [#(player+1)],
      //)},

      // round header
      table.cell(
        align: horizon,
        rowspan: data.num_rounds,
        rotate(-90deg, [Round]),
      ),

      ..for (i, row) in data.chart.enumerate() {
        (
          [#(i + 1)], // round number
          ..for (j, player) in row.enumerate() {
            if j >= (start_table * 4) and j < ((start_table + row_tables) * 4) {
              ( [#(player + 1)], )
            }
          },
        )
      }
    )
  }
}

