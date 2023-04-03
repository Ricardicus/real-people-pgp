# only Real People PGP

Building some of the building blocks for an attempt to solve the problem of chatbot overflow
caused by platform incentive structures, ai and human behaviour.

# Background

It is likely that the public internet will increasingly be driven my LLMs.
Verifying who is human is no easy task, but there are solutions that 
might be more social than technical. In this project, humans that use open PGP
are encouraged to verify themselves in the real world; where bots (yet) aren't
present. 

# Building blocks

I am building 

# A work in progress

.. with lots of things to do.
Summary this far:

* I am the current CA authority.
  * The CA authority signs your PGP key, and gives you a signed key back
  * Do you want to be a CA? Let me know. Lets meet.
  * All CA issuers are provided with name that is trackable in this project.
* There is a simple server and client program available
  * The idea is to make it possible to host multiple of these servers
  * The server checks signature validity
* Clients use a list of domain names for hosts that is provided here, but localhost should work if the server is running and you have the latest version on it (which might be hard to guarantee, but things move slow).

# Todo

* Domain names list and signature
* TLS over the middleware
* Change so that this is based on OpenPGP instead
* Domain hosting, domain list with signatures


