# Introduction

## Preface

## Conformance

## Example

``` 
{
  "@context": "https://www.w3.org/ns/did/v1",
  "id": "did:twin:123",
  "verificationMethod": [{
     "id": "did:twin:123#controller",
     "type": "Secp256k1VerificationKey2018",
     "controller": "did:twin:201:be1b:49ff:b6bc:a715:5af:6e5a:6fd3",
     "SubstrateAddress": "0xb9c5714089478a327f09197987f16f9e5d936e8a"
  }],
  "authentication": [
     "did:twin:example.com#controller"
  ]
}
``` 

# Twin DID Method Specification

## Target System

The target system of the Twin DID method is the vault of the Digital Twin, identified as twin on the on the  host that the twinID described by the DID resolves to when queried over the [Planetary Network].

## Method Name

The namestring that shall identify this DID method is: twin. A DID that uses this method MUST begin with the following prefix: did:twin. Per the DID specification, this string MUST be in lowercase. The remainder of the DID, after the prefix, is specified below.

## Method-specific identifier

The method specific identifier is a twinID as stored on [TFChain] with an optional path to the DID document.

The method specific identifier MUST match the common name used in the SSL/TLS certificate, and it MUST NOT include IP addresses. A port MAY be included and the colon MUST be percent encoded to prevent a conflict with paths. Directories and subdirectories MAY optionally be included, delimited by colons rather than slashes.

twin-did = "did:twin:" twinID
twin-did = "did:twin:" twinID * (":" path)

> EXAMPLE: Example Twin Method DIDs
> did:web:123
> did:twin:123:doc:<mydoc.json>

## DID method operations

### Create (Register)

Creating a DID is done by:

Applying at a twin for use of the linked IPv6 address as registered on TFChain and storing the location in the digital vault linked to the IPv6 vault address, creating the DID document JSON-LD file including a suitable keypair, and storing the did.json file under the TwinID vault address, or under the specified path if many DIDs will be resolved in this domain.
For example, for the twinID 123, the did.json will be available under the following URL:

EXAMPLE: Creating the DID
did:twin:123
 -> http://[1080:0:0:0:8:800:200C:417A]/did.json

### Read (Resolve)

A GET request to the ... endpoint with a DID returns the DID Document corresponding to this DID. Behind the scenes, the request queries the ledger to find the twin, gets the twin's IPv6 address, goes over ThreeFold's Planetary Network and returns the DID Document stored in a user's Digital Twin vault, if it exists.

### Update

Only the twin owner = the DID subject holder has the right to update the DID. Non-DID subject holders who initiate DID update transactions will be filtered out by the DID resolver, and will not affect the target DID in any way. When updating the DID, incremental update is not supported, and complete DID documents must be attached to the update transaction.

The rules for DID updates are as follows:

Must include one header property, wherein the operation property value is update, indicate to update a DID.
header must include the previousTxid property, and the value is the transaction ID of the previous DID document operation. Used to avoid unnecessary erroneous update operations.
Must include the payload property, and the value is the new DID document.
Must include one proof property, including the DID holder's public key reference and signature, used to prove that this operation is initiated by the twin owner.

Example: 

``` 
{
  "header": {
    "specification": "twin/did/1.0",
    "operation": "update",
    "previousTxid": "..."
  },
  "payload": "...",
  "proof": {
    "verificationMethod": "...",
    "signature": "..."
  }
}
``` 

### Deactivate (Revoke)

## Examples

The twin ID as registered on TFChain: 
``` 
      {
        "accountId": "5Cd84uMyA4VU3cGKmBurVPXNN8PSvvFxWrDzE6Uh6Tt5nDkT",
        "createdAt": "2021-10-27T13:14:48.000Z",
        "createdById": "L3k1MyQqiD",
        "deletedAt": null,
        "deletedById": null,
        "gridVersion": 1,
        "id": "Ae1ebDlLa0",
        "ip": "201:be1b:49ff:b6bc:a715:5af:6e5a:6fd3",
        "twinId": 35,
        "updatedById": null,
        "version": 1,
        "updatedAt": "2021-10-27T13:14:48.000Z"
      }
``` 

This DID document contains a public accountID by default. 

The public key #key's type is sr25915.
The public key #key's controller is the holder of the target DID subject did:twin:35.
Public key #key can be used to authenticate the identity of the DID subject holder.

## Security and Privacy Considerations

### Security Considerations

### Privacy Considerations

### DID Document Integrity Verification

### In-transit security

### Optional Path considerations 

# Reference Implementation

# References

## Normative References

[DID-CORE]

## Informative References

[Planetary Network]
