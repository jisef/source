import requests

url = "https://live.oebb.at/assets/assets/stations.json"
response = requests.get(url)
    
# Access response data
if response.status_code == 200:
    print(response.json())
else:
    print(f"Request failed with status code {response.status_code}")