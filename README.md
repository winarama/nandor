# nandor
![alt text](https://winarama.one/img/nandor.png)

Nandor is a relentless utility application that scans the log files of a given service for script-kiddie network requests and blocks associated IP addresses. Nandor has two modes of operation, **geoblock** and **logscan**.

## geoblock
geoblock blocks all IP addresses associated with a given country.
`nandor geoblock <counrty-code-list>`
`Example: nandor geoblock af cn ru`


## logscan
logscan scans service logs for script-kiddie requests and blocks associated IP addresses.
`nandor <service-name>`
`Example: nandor coolio`

##
Currently works on Ubuntu Server (with plans for Rocky Linux in 2025). 

Powered by **Rust** 🦀