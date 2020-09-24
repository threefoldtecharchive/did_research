import base64
import nacl.signing
import nacl.encoding


def generate_random_key():
    return nacl.signing.SigningKey.generate().encode(
        encoder=nacl.encoding.URLSafeBase64Encoder
    )


def get_signing_key(seed):
    return nacl.signing.SigningKey(seed, encoder=nacl.encoding.URLSafeBase64Encoder)


def get_public_key(signing_key):
    return signing_key.verify_key.encode(
        encoder=nacl.encoding.URLSafeBase64Encoder
    ).decode("utf8")


def sign(signing_key, data):
    return signing_key.sign(
        bytes(data, encoding="utf8"), encoder=nacl.encoding.URLSafeBase64Encoder
    ).decode("utf-8")


def validate_sign(signing_key, signedObject):
    public_key = get_public_key(signing_key)
    verify_key = nacl.signing.VerifyKey(
        public_key, encoder=nacl.encoding.URLSafeBase64Encoder
    )
    return verify_key.verify(base64.urlsafe_b64decode(signedObject))