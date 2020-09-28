'use strict'

const tfResolver = require('../src/index.js')
const did = 'did:tf:5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'

async function runTest(did) {
  console.log('Begin DID Resolver Test...')
  const doc = await tfResolver(did)
  console.log(JSON.stringify(doc, undefined, 2))
  return doc
}

runTest(did)