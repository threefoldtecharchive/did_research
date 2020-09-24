import json
import base64
import nacl_wrapper

from configparser import ConfigParser
from flask import Flask, jsonify, request

config = ConfigParser()
config.read("config.ini")

seed = config["DEFAULT"]["Seed"]
signing_key = nacl_wrapper.get_signing_key(seed)

app = Flask(__name__)


@app.route("/", methods=["GET"])
def index():
    return "kyc service POC"


@app.route("/kyc/create", methods=["POST"])  # Could use PUT as method.
def create():
    try:
        identity = request.json
        signedKycObject = {
            "signedKycObject": nacl_wrapper.sign(signing_key, json.dumps(identity))
        }
        return signedKycObject, 200
    except Exception as error:
        return str(error), 404


@app.route("/kyc/validate/<signedKycObject>", methods=["GET"])
def validate(signedKycObject):
    try:
        valid = nacl_wrapper.validate_sign(signing_key, signedKycObject)
        return valid, 200
    except Exception as error:
        return str(error), 404


@app.route("/kyc/public_key", methods=["GET"])
def public_key():
    try:
        data = {"public_key": nacl_wrapper.get_public_key(signing_key)}
        return data, 200
    except Exception as error:
        return str(error), 404


if __name__ == "__main__":
    app.run()