Okay, so I am wanting to go ahead and get a job in the crypto industry, and I am wanting to go ahead and work on a framework for a distributed open source hardware system to display hirable skills on github.

I think that a lot of the crypto community right now is really focused on building trustless computational solutions that dont have a lot of real world applications. Where more distributed open systems, expecially in areas of municipal infrastructure could be more practical and lead to more sustainable revenue streams that dont depend on financial speculation of underlying coins.

The main demo use cases would be something like:

- A weather station that could operate in remote enviornments, it would be able to use 2.4ghz wifi to connect with other stations nearby, and to allow quick data downloads, a van could have a node running on a desktop computer, then form a p2p connection with the station. Thus letting it download the 100's of gigabytes of data from the station network.

- A security camera application where you can have a bunch of cameras that can be recording around a house, and then wirelessly connect to a node running in the house that can store all the data long term, as well as doing analytics using various object classifaction algorithms.


In the beginning I want the framework to support multiple kinds of network connections

- High Bandwidth, Low Latency direct P2P connections over wifi 2.4GHZ. (Primarially useful for the weather station cases)
- Low Bandwidth, Low Latency metered cellular or satilite P2P connections established over the internet using an encrypted wireguard channel and a decentralized way of choosing a STUN server to establish connectivity.




As for hardware targets I want to support now
- Sensor Nodes: An android phone that runs the software as an app and has access to sensors through the usb-c port
- Compute Nodes: A regular desktop linux app 
- (LATER IMPLEMENTATION): A framework to allow easy coding of an ESP-32 to interface to other sensor nodes on the network. However, I would at least like all our technology decisions to at least be compatible with running on embedded hardware.

This does mean that it probably makes sense to do everything in terms of a rust monolith. Expecially since it has a solution like Dioxus which can handle mobile and desktop apps.

But for starters I think its a really good idea to just limit the scope as much as possible and think about how to do the following

1) Run as a mobile app on an android device that collects data from the following sensors:
- Baromoter
- Magnetometer
- Accelerometer
- Gyroscope
- Camera
(which should be enough for a proof of concept and can tell us a lot even without external sensors)

2) Have these mobile apps connect with each other and form P2P connections over local wifi.

3) Have a compute node that will intermittently connect to a single sensor node over local P2P wifi, and download all the data from all sensor nodes, even one's that it it is not directly connected to.


The big remaining questions I am thinking about is how to handle 

1) the distributed file/data storage system.

This mainly needs to store the timeseries data that comes off the sensor nodes. Which to keep things simple could be something like:
1. As the hour begins all of the sensor data is stored in memory.
2. As the hour ends, all the sensor data from all the different devices is stored in a blob, then compressed, and stored in an e2e way on a distributed file system. .
3. When the compute node comes to connect to the rest of the data, it will read the metadata info and determines how much time has ellapsed and what data it needs to download that it is currently missing. Then queries the file system to get them.

Which luckily just pushes all the difficulty on to how to handle the distributed metadata solution.

I think for now the simplest approach is to just not worry about encryption and just store the blobs using hashes on local devices.

2) How to handle the distributed metadata and control system.

I dont have a lot of good ideas on how to do this. There are some cool CRDT's that seem to show some promise. Specifically looking at the WIP prototype https://www.inkandswitch.com/keyhive/notebook/ . It looks super powerful and flexible enough to power this application. Its in the super early beta phase, but its such a good match I am considering using it even though it is in an alpha phase and might end up being a horrible idea.

Dont have a lot of good solutions for this yet. What do you think??
