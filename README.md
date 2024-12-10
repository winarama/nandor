# nandor
![alt text](https://winarama.one/img/nandor.png)

Nandor is a relentless utility application that scans the log files of a given service for script-kiddie network requests and blocks associated IP addresses. Nandor has two modes of operation, **geoblock** and **logscan**.

## geoblock
geoblock blocks all IP addresses associated with a given country.<br>  
`nandor geoblock <counrty-code-list>`<br>  
`Example: nandor geoblock af cn ru`

## logscan
logscan scans service logs for script-kiddie requests and blocks associated IP addresses.<br>  
`nandor <service-name>`<br>  
`Example: nandor coolio`

##
Powered by **Rust** 🦀

Currently works on Ubuntu Server (with plans for Rocky Linux support in 2025). 