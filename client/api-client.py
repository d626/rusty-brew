import json
import requests



if __name__ == '__main__':
    print("Hello, World!")
    reference_series = [{'duration': 15, 'temp': 55}, {'duration': 22, 'temp': 63}]
    print(reference_series)
    response = requests.post('http://localhost:8000/reference_series/test', json=reference_series)
    print(response)
