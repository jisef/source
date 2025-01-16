from threading import main_thread

import requests as rs
import json






def load_stations(station_filepath):
    with open(station_filepath, 'r') as file:
        return json.load(file)
    
def get_code_by_name(name):
    stations = load_stations("./stations.json")
    for station in stations:
        if station["name"] == name:
            return station["eva"]
    return None



def main():
    stations = load_stations("./stations.json")
    train_inf = rs.get("https://live.oebb.at/backend/info?trainNr=765&station=8100108&date=2025-01-13&time=09%3A05").json()
    start = get_code_by_name("Wels Hbf")
    stop = get_code_by_name("Linz/Donau Hbf")
    response = rs.get(f"https://live.oebb.at/backend/trip?origin={start}&destination={stop}&time=20%3A00&date=2025-01-12").json()
    print(train_inf)



if __name__ == '__main__':
    main()
    
    