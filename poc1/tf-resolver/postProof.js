const yargs = require('yargs');
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { exit } = require('yargs');
const { Vec, Bytes } = require('@polkadot/types')

const argv = yargs
    .command('submit', 'Submit a proof to the blockchain', {
        provider: {
            description: 'Kyc Provider url',
            alias: 'k',
            type: 'string',
        },
        proof: {
            description: 'Proof hash from KYC',
            alias: 'p',
            type: 'string',
        }
    })
    .help()
    .alias('help', 'h')
    .argv;

if (argv._.includes('submit')) {
    if (argv.p === '' || argv.k === '') console.log('Bad params')

    const wsProvider = new WsProvider('ws://localhost:9944')
    ApiPromise.create({ 
        provider: wsProvider,
        types: {
            KycObject: {
             provider: 'Vec<u8>',
             proof: 'Vec<u8>'
            },
			// override custom 
			Address: 'AccountId',
			LookupSource: 'AccountId'
          }
    })
    .then(api => {
        const keyring = new Keyring({ type: 'sr25519' })
        const ALICE = keyring.addFromUri('//Alice', { name: 'Alice default' })

        api.tx.templateModule
            .addKycProof(argv.k, argv.p)
            .signAndSend(ALICE)
            .then(res => {
                // console.log(res)
                console.log('successfull')
                wsProvider.disconnect()
                exit(1)
            })
            .catch(err => {
                console.log(err)
                wsProvider.disconnect()
                exit(1)
            })
    })
    .catch(err => console.log(err))
}
