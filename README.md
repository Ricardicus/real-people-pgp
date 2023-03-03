# Proof of human

Building some of the building blocks for an attempt to solve the problem of chatbot overflow
caused by platform incentive structures, ai and human behaviour.

# Building blocks

I am building a library, a client and a server that uses ECC cryptography to establish a 
chain of trust among humans who want to be able to exclude bots from commenting
or posting on a public forum.

# A work in progress

.. with lots of things to do.
Summary this far:

* I am the current CA authority.
  * You need to ask me to generate you a set of keys (in person)
  * I host none of the keys online anywhere
  * All CA issuers are provided with name that is trackable in this project.
* There is a simple server and client program available
  * The idea is to make it possible to host multiple of these servers
  * A rootcert as well as a database file is required to make the server and client programs work in accordance with the idea of this project.
* The database isn't peer to peer, it is here.
  * Clients use a list of domain names for hosts that is provided here.
* Only servers need a database
* Clients use a list of domain names provided here. 

# Todo

* Domain names list and signature
* Challenge mechanism
* TLS over the middleware
* Domain hosting, domain list with signatures


