import hashlib

def create_hash(data):
    return hashlib.sha1(bytes(data, encoding='utf8')).hexdigest()