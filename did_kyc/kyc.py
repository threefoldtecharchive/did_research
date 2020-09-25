import json
import base64
import nacl_wrapper
import hashlib_wrapper

from configparser import ConfigParser
from flask import Flask, jsonify, request

config = ConfigParser()
config.read("config.ini")

seed = config["DEFAULT"]["Seed"]
signing_key = nacl_wrapper.get_signing_key(seed)
key = nacl_wrapper.get_key(seed)

app = Flask(__name__)


@app.route("/", methods=["GET"])
def index():
    return "kyc service POC"


@app.route("/kyc/create", methods=["POST"])  # Could use PUT as method.
def create():
    try:
        identity = request.json
        kycObject = {
            "kycObject": nacl_wrapper.sign(signing_key, nacl_wrapper.encrypt(key, json.dumps(identity)))
        }
        return kycObject, 200
    except Exception as error:
        return str(error), 404


@app.route("/kyc/validate/<signedKycObject>", methods=["GET"])
def validate(signedKycObject):
    try:
        print(signedKycObject)
        validated_sign = nacl_wrapper.validate_sign(signing_key, signedKycObject)
        decrypted_kyc_object = nacl_wrapper.decrypt(key, validated_sign)
        return decrypted_kyc_object, 200
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