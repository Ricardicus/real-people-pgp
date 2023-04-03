This project has been moved to here: <a href="https://github.com/real-people-pgp/real-people-pgp.git">real-people-pgp.git</a>.


# Only Real People PGP

Creating a secure and trustable foundation for communication in a world increasingly dominated by chatbots and AI, by leveraging OpenPGP and real-world human connections.

## Introduction

The public internet is becoming more and more influenced by large language models (LLMs) and AI-driven chatbots. As these technologies advance, distinguishing between human and non-human users becomes increasingly challenging. In response to this growing concern, the Only Real People PGP project aims to create a network of verified human users who have met and established trust in person.

## Background

While technical solutions to verify human users are essential, sometimes the best approach is a combination of technology and social interaction. OpenPGP, a widely-used encryption standard, serves as the foundation of this project. By encouraging human users to meet and verify each other in the real world, we can create a more trustworthy network, ensuring that the person on the other end of the conversation is indeed human.

## Get Involved

We welcome contributions, ideas, and feedback to help us build a safer and more secure internet for everyone. Join us in our mission to create a network of verified human users and safeguard online communication against the growing presence of AI and chatbots.

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


