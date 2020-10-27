## Idea

How to manage the explorer data under form of a DID ?
- Phonebook
- Farms
- Nodes: to get info about the nodes, this fits in a doc of a DID owned by the node, can be used to mint. 

## Reflections

- Not feasible to have full nodes info on-chain, part will be on-chain and part will be off-chain. We can link both then. 
  Minimal info on-chain: all what is needed to do reservations:   
  - Capacity info (not a lot of data, is scalable)
  - Entire smart contract flow, so we don't need the explorer anymore. 
- We can the repurpose the explorer so a farmer can specify some explorer where he uploads stuff about his nodes. Explorer url can be be put in DID document on-chain. 
- For minting, there is hardware dump, = massive data blob, is not going to scale well when on-chain. To solve this: 
  - Blob to be put separately on explorer service. 
  - Link to this blob in DID document, as well as a hash of the data dump (to prove the validity)
  - A node could also run a BCDB for itself and we store any extra data in this node's BCDB. This way the nodes running in some container (isolated) to store public info and we can have a peer-to-peer network of nodes, each of them hosting their own data. 
  So 
    - DID gives info about "me" + the address
    - proof of my claims can be found
    - to access the detailed info, use a BCDB service 
  - Is there a use case that we have to tackle when the node is not reachable ? 
    - Minting: what if node is not reachable when data is fetched for calculation of the minting ? Uptime info needs to be on-chain. This could create a big data explosion (storing all heartbeats - nodeID+timestamp+uptime -, currently every 10') => to check

## Who will have a DID ? 
  
Everyone has a DID:
  - Persons
  - Farms: also stuff managed
  - Nodes
  - 3Bot as Virtual System admin
  
Map all entities in explorer to a DID with a mention of subfields that indicate what type it is: ex. It doesn’t make sense to KYC a node. We need to have a minimum set of info to store for every object. 

Farm: limited info needs to be stored, link to nodes and to owners of the farm, creates list of authorised people who can take action on this farm
Name, owner, location, wallet. Quid price ? If smart contract on-chain ? Currently price is not used. And it might make sense to have price on node level. Other options: Automated attributes : which workloads are allowed on a node (to avoid storage on nodes that have almost no storage available, …)
On-chain ? Maybe, DID as such is not immutable, link to it is immutable. 
A node can set farmer ID on a node when it boots, in command line. A node can register itself, create DID when it’s not there and set authorisation to the farmer based on DID.

Nodes info: capacity, but also current status: number of workloads, amount reserved, details for the network
If We run bcdb on every node: stuff for workloads etc, necessary, if on-chain. If all goes right, node should report the same as it is on-chain. Light info could come directly from the nodes BCDB. 

We need to define a tree so you know where you need to go for which info, possible in BCDB.

### How to access BCDBs ?

BCDB needs to be accessible and every node needs to have an accessible BCDB. 
How? 
- Yggdrasil, DHT enabling ? 
- Include Substrate Libp2p in BCDB ? Build protocol where bcdb uses libp2p 
- Yggdrasil used as transport for everything ? Then all out of the box. All in go and all OS’s supported (Linux, Mac, Windows, Android, iOS)
Problem: now BCDB relies on the explorer to route everything, we should have something more resilient. DHT for this is nice, as long as you don’t store data there. Is also how Yggdrasil does it now too, they chitchat ip’s and public keys of the nodes and discover each other this way. Then they build tree on top of the info they have.  

Conclusion : BCDB on every node

How to interconnect? Options we see: 
- Publish Yggdrasil address in DID to locate the BCDB
- BCDB can rely on a connection to a substrate node to do the routing (resolve the DID based on sequence number)

BCDB needs to implement the resolver, not too much work
Do we want routing ? Or simply resolve the DID and go directly to there ?  
Only reason for BCDBs to talk to each other is replication. But this is probably not intended to happen like this. 

ACLs can also be adapted without issue.
Now read/write/delete based on 3bot ID, can be changed to DID. You can identify yourself based on the keys in there. As we still use same keys everywhere, not issue.

### DID for 3Bot sysadmin ?

Also create a DID for the sysadmin ? Yes, give sysadmin a DID for themselves, will solve a lot of problems: 
- need to manage also phonebook info on sysadmins representing capacity
- A human can also have multiple sysadmins, 
- Sysadmin can also have multiple authorised users, only with DID you can assign multiple people to the sysadmin. Alternatively structure human ‘owning’ (one root user) the sysadmin and others being admin of the sysadmin. But if root user disappears from an organisation => issue

Or sysadmin ID = subsection of user DID ? If so then sysadmin = a service , and not a lot is needed, only linking info

Alternative: what if don’t have DID doc for sysadmin and we don’t add them as a service? 
We then throw auth for other DIDs internal in the sysadmin
Login: goes to chain, checks whether DID = valid one, checks internal database to see if it’s allowed. 
   ==> Issue: But then farms can’t refer to sysadmin as their manager

Q: is the farm owned by a person(s) or by sysadmins ? 
It should be (a) person(s) !

Farms custom sections: 
- Owner section (all DIDs of who owns) and rules (“number of owners needed to agree before I change” = multisig structure) 
- Authorisation section
- Admin section (=same?)

### Management of farm through sysadmin

List of persons 
Sysadmin can have people included which are not admins of the farm

Farm has its own DID
If changes needed to a farm’s DID, we have all info needed to verify so that’s OK
All is simply signature based verification

When sysadmins do requests to deploy workloads to a farm, how will the nodes verify it ? Public key needs to be attached to sysadmin ID ! Smart contract needs to verify that requester is who he claims he is. 

What we could do with substrate: sysadmin prepares the trx based on data input, flow which goes through 3Bot connect which has DID and private key of you (as a person), receives signed trx in secure way, signs it again, give it back to sysadmin, then posted to the chain. 
But then you need interaction, which we don’t want, sysadmin does it on your behalf ! 
=> Use derived keys which can never compromise the person’s keys?

Use different type of keys <> ed25519 as derived keys? Then key derivation, 3Bot connect app generates key pair linked to master key pair and insert into DID of sysadmin. 
Then sysadmin has own private key derived from key of the person, so it can be recovered. Works but has to be supported explicitly. 
So it can work but maybe complicated as set-up. To be further investigated. 

Without change to DID document, owner set of the farm can change, =good. We should continue to support farms changing owners. 

Should there be an explicit difference between different DIDs ? From authorisation perspective, no, all is simply signature based. Only what is inside the DID document is different. We could even bring it wider, and even support DID from elsewhere (Ethereum, …). You can have support from other DID providers in sysadmin. Only then different methods to be supported (longer term). 

Depending on what DID represents, attributes might differ, ex. KYC only for persons. But for signing that makes no difference. 

## Conclusion on DIDs

- We can convert most of what we have today in explorer into did set on-chain
- DID on all levels: farml, node, person, sysadmin
- Something needs to be done on BCDB
- Connect BCDBs through yggdrasil ? 
- Later on: info on node can be found under key x in format y. Explorer front-end can be still used as cache for DIDs which are on-chain and resolves the data inside to fetch BCDB info, becomes a resolver and cache for node info. 
- Workloads can be converted into smart contract (= what POC did)
- Directory => DIDs

## Threefold Connect

Threefold Connect app will have to integrate with DIDs as well, if we move to substrate we can have DID from user’s address ? Depends, only if you want to tie DID address to a single key.

## Which DID needs tokens ?

Substrate balances module (manages things related to funds) could be modified to be tied to DIDs, we could say ‘address which doesn’t belong to a DID could only have limited amount of 50 tokens, just enough to create transactions (and track that over lifetime not to get more than 50 tokens).
You could have DID with a list of addresses, people could have more than one address if they search use. If you add your address to DID you can have more than 50 tokens (because we know who you are). We can verify that you own the address because you need to provide a signature from the private key linked to the address, proving that you own the address. Also multi-signature is possible with one of the keys that is already in the DID and one of the keys that belong to the address.
Needs to be done, people that are KYC proven can use unlimited funds. 
Nodes, sysadmins and people will need tokens, farms won’t. Alternative is to apply sponsorship (similar to ongoing change with Stellar), substrate is configurable, so we can do this also here. 
KYC needs to happen with whom ? User and owner of the farm (so both persons).
Managing trx fees = hard part ? 
Sysadmin should be able manage its own wallet, then you can decide who pays for it. 
So only nodes = problem for trx fees ? Problem ? Right now, one of the issues = if I boot a node with farm_id, then the node is auto part of this farm with farm_id. No way to block that. Farmer to accept node in his farm ? Just a check in 0-OS, data needs to be there on the farm that the farm has accepted yourself. We can do that: we have DID of farm set in the node DID. We just need a permanent verification? 
“Accept node in farm” call is possible: You have the farm DID so you can resolve the owners, just check that it’s a signature from owners or sysadmins. Even if sysadmin who signs is later removed, no issue (all is logged in history). 
We can also fund the nodes with tokens once the farmer accepts. 

you know the key pair of the node, so you can have public key in your DID as well and you know which nodes belong to you. So in Farm UI you can see list of the nodes. Sysadmin’s wallet can keep the nodes addresses funded, can be automated. 

## How to set up the node and fund with tokens from the farmer ? 

Issue (only to automate, it can work in a manual process): Node needs to have funds to create their own DID. Unless we derive DIDs from addresses, but then we still need to set the farm DID in the node DID, so also funds needed. So: accept node, then fund node and then node can create its own DID ? 
Boot parameters? Derive key from farmer 3bot

- Farmer can specify address to the node, node generates its own address and goes to BCDB of the farmer, in a specific place, specifying “please fund me on my wallet address!” Then farmer funds the wallet and the node becomes operational. So keys can still be generated by the node only, no exchange of keys needed. Just direct communication over yggdrasil. 
   How then secure farmer’s bcdb from others’ nodes ? Farmer needs to accept/validate the info (MAC address, …), so no issue. 
   But then you need to have farmer’s BCDB accessible over yggdrasil. This can work once the app is available. And first time bootstrap on farmer’s laptop. 

Alternative options (but probably more complicated): 
- VAULT to store temporary keys, to read one time to sign the transaction, after which nodes create their own keys ? 
VAULT = central component to store keys, and you can give tied access to other parties to read specific keys. So farmer could store private keys in vault, and give permission to the node to read the key through boot parameter? Authorisation in VAULT = through generation of API key that then is used by the node. 
- give key directly as a boot parameter. Because you can revoke access. Remark then: to check whether there is a limit to passing keys in boot process. 

Issue: Now, when a node fails, when it reboots, it looses identity and creates a new identity. Farmer could run their own vault instance. 
