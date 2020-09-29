const { ApiPromise, WsProvider } = require('@polkadot/api')

// build the DID Document
async function buildDocument(did, address) {
    // Construct
    const wsProvider = new WsProvider('ws://localhost:9944')
    const api = await ApiPromise.create({ 
        provider: wsProvider,
        types: {
            Attribute: {
              name: 'Vec<u8>',
              value: 'Vec<u8>'
            }
          }
    })

	let doc = {
		'@context': 'https://w3id.org/did/v1',
		id: did
    }
    
    try {
        console.log(`trying to retrieve attributes for address: ${address}`)
        const res = await api.query.templateModule.attributes(address)

        console.log(`got res: ${res}`)

        if (res.length > 0) {
            doc.kyc = []
        }
        res.map(attribute => {
            const provider = hex_to_ascii(attribute.name).trim().replace(/\0/g, '')
            const proof = hex_to_ascii(attribute.value).trim().replace(/\0/g, '')
            doc.kyc.push({
                provider,
                proof
            })
            console.log(`provider: ${provider}`)
            console.log(`proof: ${proof}`)
        })
    } catch (err) {
        return err
    }


    doc.publicKey = [
        {
            id: `${did}`,
            type: ['SS58'],
            owner: did,
            publicAddress: address
    }]

    wsProvider.disconnect()
	
	return doc
}

// resolve the DID and return the DID document
async function didResolver(did) {
	let address
	if (!did.match(/^did:tf:[a-zA-Z0-9]{48}$/)) {
        return { error: 'Not a valid Threefold DID' }
	} else {
		address = did.split(':')[2]
	}
	return buildDocument(did, address)
}

function hex_to_ascii(str1) {
    var hex  = str1.toString();
    var str = '';
    for (var n = 0; n < hex.length; n += 2) {
        str += String.fromCharCode(parseInt(hex.substr(n, 2), 16));
    }
    return str;
}

module.exports = didResolver
