const yargs = require('yargs');
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { exit } = require('yargs');
const { Vec, Bytes } = require('@polkadot/types')

const argv = yargs
    .command('submit', 'Submit a proof to the blockchain', {
        address: {
            description: 'Address to store proof on',
            alias: 'a',
            type: 'string',
        },
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
    if (argv.a === '' || argv.p === '' || argv.k === '') console.log('Bad params')

    const wsProvider = new WsProvider('ws://localhost:9944')
    ApiPromise.create({ 
        provider: wsProvider,
        types: {
            //KycObject: {
            //  provider: 'Vec<u8>',
            //  proof: '[u8; 32]'
            //}
			// override custom 
			Address: 'AccountId',
			LookupSource: 'AccountId'
          }
    })
    .then(api => {
        const keyring = new Keyring({ type: 'sr25519' })

        // Some mnemonic phrase
        const PHRASE = 'blur sphere engine click enact output erode adult system miracle fly salad'

        // Add an account, straight mnemonic
        const test = keyring.addFromUri(PHRASE);
		const ALICE = keyring.addFromUri('//Alice', { name: 'Alice default' });

        let utf8Encode = new TextEncoder();

		// let provider = new Bytes(api.registry, utf8Encode.encode('google'));
		// console.log(provider);

        // const ob = api.createType('KycObject', utf8Encode.encode(argv.k), utf8Encode.encode(argv.p))
        // console.log(ob)
		let hash = utf8Encode.encode('FTY7sU89H9pVlFgIU8u6l2Fe9lMkmuca');
		console.log(hash)

        api.tx.templateModule
            .addKycProof("0x123456", hash)
            .signAndSend(ALICE)
            .then(res => {
                console.log(res)
                console.log('successfull')
                exit(1)
            })
            .catch(err => {
                console.log(err)
                exit(1)
            })
    })
    .catch(err => console.log(err))
}
