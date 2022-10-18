const express = require('express')
const livereload = require('livereload')
const app = express()
const port = 3000
const path = require('path')
const fs = require('fs')

const mysql = require('mysql')
const db = mysql.createConnection({
  host     : 'localhost',
  user     : 'euchre',
  password : 'euchre',
  database : 'euchre'
})
db.connect()

const liveReloadServer = livereload.createServer();
liveReloadServer.watch(path.join(__dirname, 'public'));

const connectLivereload = require("connect-livereload");

app.set('view engine', 'ejs')

app.use(connectLivereload())
app.use(express.json())

app.use(express.static('static'))

liveReloadServer.server.once("connection", () => {
  setTimeout(() => {
    liveReloadServer.refresh("*");
  }, 100);
});

app.get('/', (req, res) => {
  res.render('index', { title: 'Hey', message: 'Hello there!' })
})

app.get('/admin', (req, res) => {
  fs.readdir('euchre_fun_charts', (err, files) => {
    if (err) send_err(res, err)
    res.render('admin', {
      sizes: files.map(f => f.slice(0, -4)),
    })
  })
})

app.get('/game/:id', (req, res) => {
  let id = req.params.id
  db.query('SELECT players, name FROM game WHERE id = ?', id, (err, game) => {
    if (err) send_err(res, err)
    db.query('SELECT player.name, SUM(points) as points FROM round JOIN player ON player.game_id = round.game_id AND player.position = round.position WHERE round.game_id = ? GROUP BY round.game_id, round.position', id, (err, points) => {
      if (err) send_err(res, err)
      console.log(points)
      db.query('SELECT name, position FROM player WHERE game_id = ? ORDER BY position', id, (err, players) => {
        if (err) send_err(res, err)
        let total_players = game[0].players
        console.log(players)
        parse_chart(players, chart => {
          console.log(chart[0].games)
          res.render('game', {
            game_id: id,
            chart: chart,
            total_tables: chart[0].games.length,
            name: game[0].name,
            total_players: total_players,
            players: add_empty_players(players, total_players),
            points: points,
          })
        })
      })
    })
  })
})

app.post('/game/:id/player/:pos', (req, res) => {
  let save = [
    req.params.id,
    req.body.round,
    req.params.pos,
    req.body.point,
  ]
  db.query("REPLACE INTO round (game_id, round, position, points) VALUES (?)", [save], (err, result) => {
    if (err) send_err(res, err)
    send_ok(res, {})
  })
})

app.get('/game/:id/player/:pos', (req, res) => {
  let game_id = req.params.id
  let position = req.params.pos
  db.query('SELECT players, name FROM game WHERE id = ?', game_id, (err, result) => {
    if (err) send_err(res, err)
    let game = result[0]
    db.query("SELECT * FROM player WHERE game_id = ?", game_id, (err, result) => {
      if (err) send_err(res, err)
      let players = result;
      db.query("SELECT * FROM player WHERE game_id = ? AND position = ?", [game_id, position], (err, result) => {
        if (err || !result[0]) { send_err(res, err); return }
        let me = result[0];
        db.query("SELECT points FROM round WHERE game_id = ? AND position = ? ORDER BY round ASC", [game_id, position], (err, result) => {
          if (err) send_err(res, err)
          let points = result[0] || [];
          parse_chart(players, chart => {
            res.render('player', {
              game_id: game_id,
              position: position,
              name: me.name,
              points: add_empty_scores(points, game.players),
              chart: chart,
            })
          })
        })
      })
    })
  })
})

app.post('/game', (req, res) => {
  console.log(`Requesting new game "${req.body.name}" of size ${req.body.players}`)
  db.query("INSERT INTO game SET ?", req.body, (err, results) => {
    if (err) send_err(res, err)
    let game_id = results.insertId
    let payload = {
      game_url: `/game/${game_id}`,
      edit_url: `/game/${game_id}/edit`,
    }
    send_ok(res, payload)
  })
})

app.get('/game/:id/edit', (req, res) => {
  let id = req.params.id
  console.log('editing game ' + id)
  db.query('SELECT players, name FROM game WHERE id = ?', id, (err, game) => {
    if (err) send_err(res, err)
    db.query('SELECT name, position FROM player WHERE game_id = ?', id, (err, players) => {
      if (err) send_err(res, err)
      let total_players = game[0].players
      res.render('game_edit', {
        game_id: id,
        name: game[0].name,
        total_players: total_players,
        players: add_empty_players(players, total_players),
      })
    })
  })
})

app.post('/game/:id/edit', (req, res) => {
  let id = req.params.id
  console.log('updating players for game ' + id)
  let players = []
  req.body.players.forEach(player => {
    players.push([id, player.name, player.position])
  })
  console.log(players)
  let query = db.query('REPLACE INTO player (game_id, name, position) VALUES ?', 
    [players],
    (err, result) => {
      if (err) send_err(res, err)
      send_ok(res, {})
    }
  )
})

add_empty_scores = (scores, total) => {
  for (let i = scores.length; i < total; i++) {
    scores.push(0)
  }
  return scores
}

add_empty_players = (players, total) => {
  for (let i = players.length; i < total; i++) {
    players.push({
      name: "",
      position: i + 1,
    })
  }
  return players
}

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})

send_err = (res, error, payload={}) => {
  console.log(error)
  res.send(JSON.stringify({
    error: error,
    ...payload,
  }))
}

send_ok = (res, payload) => {
  res.send(JSON.stringify({
    ...payload,
  }))
}

parse_chart = (players, callback) => {
  let pos_to_name = {}
  players.forEach(player => {
    pos_to_name[player.position] = player.name
  })

  let num = players.length
  fs.readFile(`euchre_fun_charts/${num}.txt`, (err, data) => {
    if (err) console.log(err)
    let rounds = []
    data.toString().split("\n").filter(x => x).forEach(line => {
      let parts = line.split(";")
      let byes = parts[1].split('+').map(x => pos_to_name[x])

      let games = parts[0].split(',')
      games = games.map(game => {
        let teams = game.split('v')
        return teams.map(team => {
          players = team.split('+')
          return players.map(x => pos_to_name[x])
        })
      })
      rounds.push({
        byes: byes,
        games: games,
      })
    })
    callback(rounds)
  })
}
