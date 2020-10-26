# Idea

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
### Who will have a DID ? 
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
We need to define a tree so you know where you need to go for which info, possible in BCDB

### How to access BCDBs ?

BCDB needs to be accessible and every node needs to have an accessible BCDB. 
How? 
- Yggdrasil, DHT enabling ? 
- Include Substrate Libp2p in BCDB ? Build protocol where bcdb uses libp2p 
- Yggdrasil used as transport for everything ? Then all out of the box. All in go and all OS’s supported (Linux, Mac, Windows, Android, iOS)
Problem: now BCDB relies on the explorer to route everything, we should have something more resilient. DHT for this is nice, as long as you don’t store data there. Is also how Yggdrasil does it now too, they chitchat ip’s and public keys of the nodes and discover each other this way. Then they build tree on top of the info they have.  
Conclusion : 
- BCDB on every node
- Publish Yggdrasil address in DID to locate the BCDB
- BCDB can rely on a connection to a substrate node to do the routing (resolve the DID based on sequence number)

BCDB needs to implement the resolver, not too much work
Do we want routing ? Or simply resolve the DID and go directly to there ?  
Only reason for BCDBs to talk to each other is replication. But this is probably not intended to happen like this. 

ACLs can also be adapted without issue.
Now read/write/delete based on 3bot ID, can be changed to DID. You can identify yourself based on the keys in there. As we still use same keys everywhere, not issue.

### 3Bot sysadmin 

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

Farms: 
- have owner section and rules (“number of owners needed to agree before I change” = multisig structure) 
- Have admin section

