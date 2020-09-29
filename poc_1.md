# POC 1

The aim of this POC is twofold: investigate if and how we can use a substrate
based blockchain to store (general purpose) DID's, and if we can use those
DID's together with some form of KYC procedure. Furthermore, we want to use DID's
as a secure login method, and services should optionally be able to verify KYC'ed
information of a customer.

## Substrate based DID

First of all, it should be noted that [the DID spec](https://www.w3.org/TR/did-core/)
does not require that the document itself is stored, but rather that some data
is stored such that a _DID Resolver_ can read this data, and create a full
DID document from it (given the DID URL as input). We can thus store individual
pieces of data, which can be combined to deliver the full document. The data itself
can actually be rather complex since it is not strictly key value. A claim is allowed
to have sub objects itself. This somewhat complicates the representation of a
value. Because the main goal was to investigate the KYC flow, we did not spend
a lot of time on this, and elected to go with a simple key/value approach for now.

Substrate itself gives the developer access to a key-value storage. Operations
on this storage are done through extrinsics, which can be thought of as transactions.
At the same time, an extrinsic can be thought of as an api call. There are some
arguments, defined in the pallet (a module), and then some operations, which may
take these arguments, and modify the current storage (the state of the system).
Thinking about this way, a substrate blockchain can be thought of as an immutable
list of past API calls, which modified the system state, such that replaying the
entire list of calls yields the current state. Calling an extrinsic requires payment
of a small fee. This provides economic transaction for the chain. Although this
could be disabled, it is not recommended. Part of this is because an extrinsic
is only executed when the block is made and imported. When the extrinsic is submitted,
only the "envelope" is validated. That is, signature validation (if present), argument
type and count, etc. The code specified for the extrinsic is not actually run yet.
This only happens on block import, as said previously. As such, validation in the
extrinsic code itself can still cause it to fail. But it will be included in a block
nonetheless, and therefore take up space in the db, since a node needs to maintain
a list of all the past blocks. So a bad actor could spam valid extrinsics which
contain garbage data, thus wasting processing power and storage space.

Retrieval of data (reading) can be done without extrinsics. The main way to do
this is to query the storage of a node. This gives access to raw data. It is then
the responsibility of the application on top to reassemble and decode the data
so that it becomes meaningful again. Because of the simple structure of the data,
this is the approach we've taken in the POC. Do note that some libraries, which
include the javascript library, make this significantly easier. Another option
is the implementation of custom rpc calls. This could be made to retrieve the
DID data given the url, and assemble and return the full document. This approach
would mean that any substrate node is essentially a resolver (given that you can
query it using the substrate RPC).

## DID login and verified information

Logging in with a DID is rather straightforward. Given a DID URL, the third party
can fetch the document. This document can then list a number of controllers, which
can be public keys. A login attempt can generate a challenge which must then be
signed by one of the keys. If a valid signature is provided, the DID is authenticated.
By allowing specification of multiple keys, there is also out of the box support
for representation of non-human entities. For instance, an organization can list
multiple keys of its members, who are authorized to log in and take action on behalf
of the organization. Also note that the challenge flow seems similar to what already
exists now in the 3bot connect app.

For the kyc object, a lot will depend on how the KYC provider operates and if
they integrate with us at all. For operation, the 2 main methods would be retrieval
of personal data with the object, or verification of provided data with the object.
Overall this leads to the following flows:

- The object allows retrieval of data:
  - At this point, there needs to be a scope mechanism which defines which pieces
  of data the service should be allowed to access. If the provider does not integrate
  with us, there seems to be no real way to enforce this, since the service could,
  given the object, forge a request to the provider (again, no integration so the
  provider won't check our signatures). The object needs to be kept in private storage,
  and only a hash should be published, to ensure the object is owned by this user.
  To avoid impersonation, the hash must be unique. This approach will thus not work,
  since malicious services can not really be stopped from abusing the object once
  they have it.

  - If there is an integration of some sorts, we can set up a scope mechanism which
  would only allow partial access to the KYC data. With this approach, we can
  store the actual object on chain, since the client still needs to create a signed
  request with the right scopes. Upon (first) login, the client would need to perform
  an additional challenge with the service, where the service requests access to
  some scopes. This flow is familiar to the old IYO JWT flow.

- object allows verification of data.
Here, there is no real difference between a service which integrates with us or not.
In either case, the object should be storable on chain (unless it contains personal
data). Personal data is stored locally (e.g. inside 3bot connect). When a user logs
in (for the first time), he must also provide the requested personal data. Since
the KYC object is part of the DID document, the service can extract it, and query
the KYC provider with the provided object and personal data. The provider then
verifies that the data is indeed valid for the person who got this object. This
does require that the personal data is stored locally on all devices a user can
log in with, though that should not be a problem since we only have one. Also,
the request for personal data can be included in the login flow for first time
log ins.

As said, all of this is dependant on which KYC provider we will eventually choose.

## Notes

Blockchain based DID's do lead to some concerns, since a user needs to pay a small
fee to modify them. Evidence has shown that typical end users are bad with managing
secrets (and associated value), possibly because they have no prior experience with
this.

Starting a whole new blockchain just to maintain DID's is probably not really a
good idea. However it might be interesting if we can also get the smart contract
for IT on there.
