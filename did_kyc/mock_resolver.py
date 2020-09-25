import json
import base64

from flask import Flask, jsonify, request

app = Flask(__name__)

@app.route("/did/<did>", methods=["GET"])
def did(did):
    try:
        did_json_dummy_response = {
            "@context": "https://w3id.org/did/v1",
            "id": "did:tf:5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
            "kyc": [{"provider": "http://127.0.0.1:5000/kyc/validate/", "proof": "FXynTD-aztIE6xy40JPqYtu-E8awToZ4XUCrnTPxsDqOCtiZIuyhnXNNMtwFT8ENyzTFZZsA1b34sJrTpiEEAnsiZmlyc3RfbmFtZSI6ICJNYXRoaWFzIiwgImxhc3RfbmFtZSI6ICJEZSBXZWVyZHQiLCAicHJvb2YiOiAiQUJDMTIzIn0="}],
            "publicKey": [
                {
                    "id": "did:tf:5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                    "type": ["SS58"],
                    "owner": "did:tf:5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                    "publicAddress": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                }
            ],
        }
        return did_json_dummy_response, 200
    except Exception as error:
        return str(error), 404

if __name__ == "__main__":
    app.run()