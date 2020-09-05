# Dignitas

###### Main repository of the dignitas project.

### Components:

- Hyperledger Sawtooth

    To launch the Distributed Ledger nodes, the Consensus, and the Ledger API
	
	**Pre Requisites**
	- Docker & Docker Compose [link](https://docs.docker.com/compose/install/)
	
	**Instructions**
```
$ cd docker
$ docker-compose up --force
```
    
- Transaction Processor
	
    Business logic in order for the Sawtooth nodes understand and process our requests

	**Pre Requisites**
	- Rust [link](https://www.rust-lang.org/learn/get-started)

	**Instructions**
```
$ cd tp
$ cargo run
```

- Gateway Server
	
    Server that acts as a gateway between mobile nodes and the Sawtooth Network
	
	**Pre Requisites**
	- Rust [link](https://www.rust-lang.org/learn/get-started)

	**Instructions**
```
$ cd ledger-proxy
$ cargo run
```
    
 - CLI
 
 	Simple program that enables the testing of the dignitas features
	
	**Pre Requisites**
	- Rust [link](https://www.rust-lang.org/learn/get-started)

	**Instructions**
```
$ cd cli
$ cargo run -- -h # to see the help for this cli
```

## Components in other repositories
	
-  [dignitas-app](https://github.com/LudeeD/dignitas-app)

	A android application (Kotlin) that should be used by **untrusted** third parties to interact with the system
    
 - [dignitas-app-trusted](https://github.com/LudeeD/dignitas-app-trusted)
 
 	A NodeJS application that should be used by **trusted** third parties to interact with the system
