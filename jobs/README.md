# Jobs

A little PoC to experiment with distributed job queues.


## Design

We use the following ports to and protocols to achieve a robust cluster.
1. 6000:
  a. Survey - used for leader election
  b. Pub/Sub - used for state updates/verification and heartbeats
2. 6001:
  a. Req/Rep - used for generic comms


### Surveying

Electing a leader is priority number one, it works as follows:

1. Everyone tries to connect to everyone.
2. Find out who has the most peers.
  a. Most recent agreed on transaction.

3. Peers connect to winner in Pub/Sub mode.

```yaml
srvr0:
  leader: srvr0
  log:
    - id: 1234
      timestamp: 0
    - id: 5678
      timestamp: 1

srvr1:
  leader: srvr0
  log:
    - id: 1234
      timestamp: 0

srvr2:
  leader: srvr0
  log:
    - id: 1234
      timestamp: 0
```

#### Clean

```yaml
born: 2021-01-01T00:00
leader: null
log: []
name: "srvr0"
peers: 2
```

We use the following:
  1. peers
  2. uptime
  3. name
