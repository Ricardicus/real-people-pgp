import pohlib

k = pohlib.KeyMaster()
print("Secret key: {}".format(k.secret_key))
print("Public key: {}".format(k.public_key))

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


