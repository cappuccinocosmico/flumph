
Okay, so one of the things I am curious about is if there is a better
  way of figuring out the following.


I have a chunk of data, and I want to store it in a decentralized wayon another server. I want to make sure that no one else can read it. (ie its e2e encrypted) But I also really want to ensure that if I store the same data multiple times it uses the same storage space.

There are some methods of securely doing this. Namely if you dont care about encryption you can use a system like IPFS, which is just a combination of a distributed hash table that stores the hashes of all your blobs. However this data is typically stored unencrypted, and while you can store encrypted data it using a following protocol

1. Take a set of data D, and calculate its hash H(D). Then encrypt D using a symmetric key algorithm giving you AES(D, H(D)). You then store AES(D, H(D)) on an IPFS style system. Storing the following 2 hashes and concattenating them together (H(D), H(AES(D,H(D)))). You can then retrieve your data using the last half of the final key, then decrypt it with the first half.

The main thing I want this distributed data system for is likely to be a really small distributed system of no more than 30-40 nodes. And it would also be really nice if it could run on an embedded system, which I dont know if IPFS could do. I dont want to fall victum to "invented here syndrome", are their any existing solutions that can handle this? Or should I just implement something real quick in rust.


The main thing this is going to get used for is in  /Users/nicole/Documents/flumph/main_idea.md
 if you need more context.
