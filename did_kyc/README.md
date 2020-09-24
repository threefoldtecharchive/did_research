# kyc example

This is a dummy KYC service to create a POC. This POC currently returns the entire sign which also contains data which is READABLE to anyone, this could later on be replaced by an id or hash which links to a record kept in a database.

## Features

* Create a KYC signed object
* Validate a KYC signed object
* `Makefile` for automating common development tasks:
    - Installing dependencies
    - Running the app with `flask`

## Installing dependencies

```
$ pip install -r requirements.txt
```

or 

```
$ make install
```

## Usage

```
$ python3 kyc.py
```

or 

```
$ make run
```


## Endpoints

When each endpoint was hit successfully it will return a HTTP_200 otherwise it will be a HTTP_404 with the corresponding error.

# Create

Request

```
POST http://127.0.0.1:5000/kyc/create

{
    "first_name": "myFirstName",
    "last_name": "myLastName",
    "proof": "proofThatIam"
}
```

Response

```
{
    "signedKycObject": "1YxCe4Co872UJcmzgZ-y3NrqSxLNhOVtjFBfCLJBI9Hj2cUb1al55cjk3dktG41ZmF0OTAd3S2PmREyx4WkPB3siZmlyc3RfbmFtZSI6ICJteUZpcnN0TmFtZSIsICJsYXN0X25hbWUiOiAibXlMYXN0TmFtZSIsICJwcm9vZiI6ICJwcm9vZlRoYXRJYW0ifQ=="
}
```


# Validate

Request

```
GET http://127.0.0.1:5000/kyc/validate/FXynTD-aztIE6xy40JPqYtu-E8awToZ4XUCrnTPxsDqOCtiZIuyhnXNNMtwFT8ENyzTFZZsA1b34sJrTpiEEAnsiZmlyc3RfbmFtZSI6ICJNYXRoaWFzIiwgImxhc3RfbmFtZSI6ICJEZSBXZWVyZHQiLCAicHJvb2YiOiAiQUJDMTIzIn0=
```

Response

```
{"first_name": "myFirstName", "last_name": "myLastName", "proof": "proofThatIam"}
```


# Public key

Request

```
GET http://127.0.0.1:5000/kyc/public_key
```

Response

```
{"public_key":"y6kdT2eWfjKy-kU-6ES8yn1T0FbFyCymxTnGCl2ekQs="}
```

