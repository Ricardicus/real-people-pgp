import pohlib

def print_keys(k):
    print("Secret key: {}".format(k.secret_key))
    print("Public key: {}".format(k.public_key))
    print("Pass: {}".format(k.passphrase))

def keys_hash(k):
    return pohlib.hash_string(k.secret_key + k.public_key + k.passphrase)

k = pohlib.KeyMaster("My passphrase")
print_keys(k)

public_key = k.public_key

c = "hejsan"
signature = k.sign(c)
certificate = (c, signature)

print("Message: {}".format(c))
print("Certificate ({}, {})".format(c, signature))
print("Valid: {}".format(k.verify_with_public_key(public_key, c, signature)))

new_cert = k.new_certificate()
print("New certificate: ({},{})".format(new_cert[0], new_cert[1]))
print("Valid: {}".format(k.verify_with_public_key(public_key, new_cert[0], new_cert[1])))

k.export_to_file("test-file")

k2 = pohlib.KeyMaster()
k2.import_from_file("test-file", "My passphrase")
print_keys(k2)
print(keys_hash(k2))

print(keys_hash(k2) == keys_hash(k))
