#let data = json("../py_euch/data/chart.json")
#let players = data.players

#let num_tables = calc.floor(data.num_players / 4)

#let player = players.at("0")
#let num_rounds = player.len()

#set page(columns: 2, margin: 0.4in)
#set columns(gutter: 0.8in)

#set table(inset: (y: 0.6em))

// #show table.cell.where(y: 0): body => box(fill: blue, rotate(-90deg, reflow: true, body))

#for i_player in range(players.len()) [
  #let player = players.at(str(i_player))
  #{
    set text(size: 1.5em)
    [*Player #{i_player + 1}:*]
  }
  // #box(width: 1fr, line(start: (10pt, 0pt), length: 100% - 10pt))

  #table(
    //columns: (1fr,) * 3 + (2fr, 1fr, 1fr),
    columns: (auto,) * 4 + (2fr, 1fr),
    align: (right,) + (center,) * 5,
    stroke: (x, y) => if x > 0 and y > 0 {
      black
    },
    [],
    [Table],
    [With],
    [Against],
    [Score],
    [Loners],

    ..for i_round in range(num_rounds) {
      let round = player.at(i_round)
      (
        [#{ i_round + 1 }],
        [#{ round.table + 1 }],
        [#{ round.partner + 1 }],
        [#{ round.opponents.at(0) + 1 } + #{ round.opponents.at(1) + 1} ],
        [],
        [],
      )
    },
  )

  #v(1fr)

  #table(
    columns: (1fr, 1fr),
    align: center,
    stroke: (x, y) => (top: black),
    gutter: 1em,
    [Final score], [Total loners],
  )
  #v(1fr)
  #if calc.rem(i_player, 2) == 1 {
    colbreak()
  } else { }
]
