# Discussion around DID
August 3, 2020 

Present: Christophe, Dylan, Lee, Azmy, Maxime, Jan, Geert

## Topics to address during this Sprint : 
- Solve ‘what is identity on the grid’ : DID implementation ?
- How to decentralize data held in the explorer (directory info for nodes and reservation info)

## DID implementation : 

### Q1: is DID good solution for our purpose ? 

- Isn’t it already similar to what we already have userid’s and public keys where you can verify signatures ? so you are already sure who owns the secure key ?
- Does it solve solution of decentralisation? 

A: Yes, it’s all about decentralisation
- Identity is all about you and you only. 
- Unlike today, if 1 person wipes the explorer DB, all IDs, public keys, ... have gone. 

DID contains
- Decentralized storage system (?)
- Blockchain (immutable, always append ledger) 

DID resolves to something

On the blockchain, hash points to sthg which is stored in distributed way. 
It’s about having the blockchain to prove that:
   - It’s owned by you
   - Can’t be changed by anyone

DID method defines how to adapt the value of the contents. 
Simplified version : verifiable credential

Blockchain is not meant to contain: 
   - Big amounts of data
   - No personal info

### Q2 : Does data need to be verifiable ? 
Hash in the blockchain is enough. 
But does not prevent data to be deleted. Addressed in the spec ? How to solve ? 

A: to be checked further whether spec’d or do we need to address this ? 
Verifiable data registry, needs to hold also data to produce DID document.  
All the data needs to be available, how to do this ? 

### Q3 : If data is not there anymore, then data is broken. So what ? 
Is it a problem ? Not if it is about your own data. 
But then what if it is about explorer data ? 

A: idea is also to decentralize the explorer data and link it to the farmer. Yes, every user maintains his own data. 

### Q4 : Current problem: 
Now we use explorer as routing table, you need to have id to know where to go and you need to know where to go to verify the identity. = biggest problem if we want to use bcdb as it is for storing the data. 

In DID document: entry for services, via service end points
So: DID on blockchain, to be used as explorer linking to BCDB service showing the document. 

### Q5 : What is identifier for DID document  ? 
- IP (+ ID of the object) ? If BCDB IP changes, then key can change (ex. When farmer stops service or container needs upgrade)… 
- DNS on top ? 
   - You can create a DNS but you need to do that before creating DID
- 3Bot ID : centralized … 

Minimal set of information needed with metadata and link to services / go and fetch the data

### Q6 : Ways to update the document ? 
By applying a set of changes, under form of an immutable log in a ledger, = delta’s resulting in the latest state. 

Resolver should not be centralized, there’s infrastructure to put in place to run it ‘on behalf of’ because not all will run their own resolver. 
Our grid is able to run an flist with this in it. 

Biggest challenge of DIDs is to have a UX for people to use it. Whole ecosystem to put in place. Luckily we already have 3Bot connect and other 3Bot interfacing, key mgmt is ok on our side, but we need a smooth UI to make it usable. 

### Q7 : Where to store the DID info ? Stellar has only 32 bytes available, might be too small to store a DID.
Possibilities: 
- Other blockchain to use ? There is no TFT on other blockchains than Stellar, so we need to implement then also other tokens to register the DID. 
  Solutions : 
    - Accept usage of different cryptocurrency (own or from a partner)
    - Implement TFT also on that separate blockchain (ERC20?)
    - Other frameworks like Hyperledger ? 
        - Advantage : no need for a separate cryptocurrency or to implement TFT on it, operation can be free when framework in place.
        - Disadvantage: it’s a permissioned blockchain, not fully decentralized, number of participants limited to the designated ones, we need separate rules to appoint the validators of a blockchain network using the Hyperledger framework. 
    - Revive Rivine ?
        - Adv: we are free to design
        - Disadv: we just abandoned it, will again bring maintenance up and infrastructure need needs to be maintained
    - Use IPFS as extended storage for the 32 available bytes on Stellar, see [Using IPFS as memo in Stellar transaction](http://abcxyz.de/2018/02/08/using-ipfs-as-memo-in-stellar-transaction/)
    - JWT ? 
    - If DID document completely stored on blockchain, no issue, every trx has an identifier based on the block. But then how to update the document? 
DID could be identifier of the created document. If update, you refer to the ‘creator’ ID

Verifiable data registry = what we can provide for people

### Q8 : What to use as identifier ? 
Possibilities:
- 3Bot, is centralised !
- DID as primary identifier ? 
- Use the public keys as identifiers (they are unique) ?

### Q9 : How verifiable does the document need to be ? And how do other solutions solve this ? 

If you store doc in the BCDB, doc needs to be produceable from data stored in a verifiable data registry. 
BCDB is not verifiable registry ? 
How does the verification happen ? To be investigated
-The method defines the verification

Answers found so far: 
- ElastOS: sidechain (so a blockchain), 
- Many others: IPFS, 
- Some simply don’t store personal data
- Did-web just points to a domain (not verifiable)

### Q10: How could bootstrap process look like ? 
First time that you create your DID, you store it first on phone. It allows the service to verify on the phone to create the BCDB and then update to IP to BCDB. 
Then if update, refer to hash of the previous delta. 

Issue on Stellar: field too small (see above - Q7 - possible solutions)

### Q11 To conclude : what is our biggest issue now ? 
Most important thing is to be able to have a DID and a doc stored somewhere without having a 3Bot, unless we use sthg else as an identity.

If you use id as identifier for 3Bot you also need a doc stored somewhere, or at least data to retrieve the document. 

You can have service integrated in wallet app, then create doc, you have your id and then with ID make reservation for bcdb and then update document to have bcdb as a service in there. 
If we have blockchain , no issue. 
Problem now we have is indirection. 
Solve through IPFS ? 
Alternative: distributed DNS ? Unstoppable domains ? 



## Video recording:
- [Part 1](https://tube.zaibon.be/videos/watch/ce39f509-3bfa-4aa9-91a6-8d25e1cb0f45)
- [Part 2](https://tube.zaibon.be/videos/watch/01bf8f82-efc6-4cd0-946a-655daf7b9816)
