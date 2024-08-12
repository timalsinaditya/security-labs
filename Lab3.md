# LAB-3 Intercepting Communciation from PWN College

## [Press here to return to HOME](index.md)

## Intrecepting Communications

### Level 1
```bash 
# just use ncat 
nc 10.0.0.115 31337
```
### Level 2 
```bash 
# Listen to the TCP Port
nc -l 31337
```

### Level 3 
```bash 
# Use ncat to scan the local network to find the host with port 31337 open

nmap -p 31337 10.0.0.0/24

# it takes little bit of time and returns the multiple ip addresses

# you just need to select the host with STATE open
nc 10.0.0.115 31337 # this was the target ip in my case
```

### Level 4 
This is similar to Level 3
```bash 
nmap -p 31337 10.0.0.0/16

#it takes more than half hour to scan all the ip addresses
nc <target_ip> 31337
```

### Level 5 
```bash 
# -X is used to print the data in both HEX and ASCII format 
# scan the data coming from all the available interface to specified port

tcpdump -X -i any port 31337
```

### Level 9
```bash
#use scapy 
scapy

#Creating ethernet frame specifying the src mac address and default dest mac
>>> ether_layer = Ether(src=get_if_hwaddr("eth0"))

#Creating IP packet
>>> ip_layer = IP(proto=0xFF, dst="10.0.0.3")

#Combime frame and ip packet into single packet
>>> packet = ether_layer / ip_layer

#send the packet
>>> sendp(packet, iface="eth0")
```