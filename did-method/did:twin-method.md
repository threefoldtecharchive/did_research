# Introduction

## Preface

## Conformance

## Example

``` 
{
  "@context": "https://www.w3.org/ns/did/v1",
  "id": "did:twin:example.com",
  "verificationMethod": [{
     "id": "did:twin:example.com#controller",
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

## Method Name

## Method-specific identifier

## DID method operations

### Create (Register)

### Read (Resolve)

### Update

### Deactivate (Revoke)

## Security and Privacy Considerations

### DID Document Integrity Verification

### In-transit security

### Optional Path considerations 

# Reference Implementation

# References

## Normative References

[DID-CORE]

## Informative References

[Planetary Network]
