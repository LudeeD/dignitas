# Dignitas

###### Main repository of the dignitas project.

### Components:

- Hyperledger Sawtooth

	To launch the Distributed Ledger nodes, the Consensus, and the Ledger API
    
- Transaction Processor
	
    Business logic in order for the Sawtooth nodes understand and process our requests

- Gateway Server
	
    Server that acts as a gateway between mobile nodes and the Sawtooth Network
    
 - CLI
 
 	Simple program that enables the testing of the dignitas features

## Components in other repositories
	
-  [dignitas-app](https://github.com/LudeeD/dignitas-app)

	A android application (Kotlin) that should be used by **untrusted** third parties to interact with the system
    
 - [dignitas-app-trusted](https://github.com/LudeeD/dignitas-app-trusted)
 
 	A NodeJS application that should be used by **trusted** third parties to interact with the system
