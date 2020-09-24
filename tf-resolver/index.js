const express = require('express')
const app = express()
const port = 3000
const threefoldResolver = require('./src/index')

app.get('/:did', (req, res) => {
  threefoldResolver(req.params.did)
    .then(doc => res.send(doc))
    .catch(err => next(err))
})

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`)
})