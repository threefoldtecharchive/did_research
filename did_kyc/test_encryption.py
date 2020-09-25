import nacl_wrapper
from configparser import ConfigParser

config = ConfigParser()
config.read("config.ini")

seed = config["DEFAULT"]["Seed"]
key = nacl_wrapper.get_key(seed)

original_text = "this is a line of text!"
print("Original text: %s" % original_text)

cipher_text = nacl_wrapper.encrypt(key, original_text)
print("Cipher text: %s" % cipher_text)

plain_text = nacl_wrapper.decrypt(key, cipher_text)
print("Plain text: %s" % plain_text)