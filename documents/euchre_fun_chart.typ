#set page(flipped: true)

#let data = json("../py_euch/data/chart.json")
#let chart = data.chart

#let num_tables = calc.floor(data.num_players / 4)

// #show table.cell.where(body: [5]): strong
//#set table(
//  fill: (x, y) => if (y < 1 and x > 1) or (x < 2 and y > 0) { gray.lighten(50%) },
//  stroke: (x, y) => if x < 1 and y == 0 {
//    none
//    //} else if calc.rem(x + 2, 4) == 2 and x > 1 {
//    //    (
//    //        top: 0.5pt,
//    //        bottom: 0.5pt,
//    //        left: 1pt,
//    //        right: 0.5pt,
//    //    )
//  } else {
//    0.5pt + black
//  },
//)

// euchrefun style

#set text(font: "Liberation Sans", weight: "bold", size: 12pt)
#set table.hline(stroke: 0.5pt + black)
#set align(center)

#text(
  size: 1.8em,
  [#data.num_players PERSON EUCHRE ROTATION],
)

#table(
  columns: calc.floor(num_tables * 3) + 1,
  rows: data.num_rounds + 1,
  align: center + horizon,
  inset: ((0.4em,) + (0.4em, 0.2em, 0.4em) * num_tables),
  column-gutter: ((6pt,) + (auto, auto, 6pt) * num_tables),
  stroke: 0.5pt + black,
  // stroke: (x, y) => {
  //   if y > 0 and calc.rem(x, 3) == 1 {
  //     (left: 0.5pt + black)
  //   }
  // },
  table.hline(),
  [Game],

  // row headers
  ..for (i, euchre_table) in range(num_tables).enumerate() {
    (table.cell(align: center, colspan: 3, [Table #(i + 1)]),)
  },
  table.hline(),

  ..for (i, row) in data.chart.enumerate() {
    (
      [#(i + 1)],
      ..for (me, ahead, left, right) in row.chunks(4, exact: true) {
        (
          [#(me + 1)-#(ahead + 1)],
          table.cell(text("vs", size: 0.5em, weight: "medium")),
          [#(left+1)-#(right+1)],
        )
      },
      table.hline(),
    )
  }
)

//#table(
//  columns: data.num_players + 2,
//  align: left,
//  inset: 0.4em,
//  column-gutter: ((auto,) + (6pt, auto, 6pt, auto) * num_tables),
//  table.cell(colspan: 2, []),
//  // row headers
//  ..for (i, euchre_table) in range(num_tables).enumerate() {
//    (table.cell(align: center, colspan: 4, [Table #(i + 1)]),)
//  },
//  table.cell(align: horizon, rowspan: data.num_rounds, rotate(-90deg, [Round])),
//  ..for (i, row) in data.chart.enumerate() {
//    (
//      [#(i + 1)],
//      ..for player in row {
//        ([#(player + 1)],)
//      },
//    )
//  },
//)
