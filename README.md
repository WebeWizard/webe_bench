### Usage

Currently implementation will always re-use connections if still open (keep-alive)

### Setup

Using rust nightly due to bug compiling libc  

Set up the mysql docker container using `docker-setup-<env>.sh`  
Tear down the mysql docker container using `docker-remove-<env>.sh`

### System Requirements
libssl-dev (ubuntu) or openssl-dev (others)