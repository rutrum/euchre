#let data = json("../py_euch/data/chart.json")
#let players = data.players

#let num_tables = calc.floor(data.num_players / 4)

#let player = players.at("0")
#let num_rounds = player.len()

#set page(columns: 2, margin: 0.5in)

// #show table.cell.where(y: 0): body => box(fill: blue, rotate(-90deg, reflow: true, body))

//#show table.cell.where(y: 0): it => {
//  let (body, ..fields) = it.fields()
//  return it.func().with(body: rotate(-90deg, reflow: true, body))(..fields)
//}

#for i_player in range(players.len()) [
  #let player = players.at(str(i_player))
  Scorecard for player #{i_player + 1}.

  #table(
    //columns: (1fr,) * 3 + (2fr, 1fr, 1fr),
    columns: (auto,) * 6,
    align: center,
    [Round],
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
        [#round.partner],
        [#round.opponents.at(0) + #round.opponents.at(1)],
        [],
        [],
      )
    }
  )
  #v(1fr)
  #if calc.rem(i_player, 2) == 1 {
    colbreak()
  } else { }
]
