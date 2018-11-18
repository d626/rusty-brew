import json
import requests

# TODO: Handle errors from the server

# TODO: rename to BrewServer?
class RustyBrew:
    def __init__(self, server_address):
        self.address = server_address

    def get_list_of_logs(self):
        route = [self.address, "logs"]
        response = requests.get("/".join(route))

        try:
            response.raise_for_status()
        except error:
            raise error

        return response.json()


    def get_log(self, name):
        route = [self.address, "logs", name]
        response = requests.get("/".join(route))

        try:
            response.raise_for_status()
        except error:
            raise error

        return json.loads(response.text)


    def delete_log(self, name):
        print("Deleting log: ", name)
        route = [self.address, "logs", name]
        response = requests.delete("/".join(route))
        print(response)

        try:
            response.raise_for_status()
        except error:
            raise error

        # No return value


    def get_current_values(self, resource):
        route = [self.address, resource, "values"]
        response = requests.get("/".join(route))

        try:
            response.raise_for_status()
        except error:
            raise error

        return json.loads(response.text)

    def get_list_of_resources(self):
        route = [self.address, "resources"]
        response = requests.get("/".join(route))

        try:
            response.raise_for_status()
        except error:
            raise error

        return response.json()


    def get_list_of_reference_series(self):
        route = [self.address, "reference_series"]
        response = requests.get("/".join(route))

        try:
            response.raise_for_status()
        except error:
            raise error

        return response.json()


    def get_reference_series(self, name):
        route = [self.address, "reference_series", name]
        response = requests.delete("/".join(route))

        try:
            response.raise_for_status()
        except error:
            raise error

        return json.loads(response.text)


    def delete_reference_series(self, name):
        route = [self.address, "reference_series", name]
        response = requests.delete("/".join(route))

        try:
            response.raise_for_status()
        except error:
            raise error

        # no return value


    def post_reference_series(self, name, reference_series):
        print("reference series to be sent: ", reference_series)
        route = [self.address, "reference_series", name]
        response = requests.post("/".join(route), json=reference_series)

        try:
            response.raise_for_status()
        except error:
            raise error

        # no return value


    def start_controlling(self, resource, profile):
        route = [self.address, "start", resource, profile]
        response = requests.get("/".join(route))
        try:
            response.raise_for_status()
        except error:
            raise error

        # no return value


    def get_log_as_csv(self, logname):
        if not logname in self.get_list_of_logs():
            return False # Raise exception instead?
        log = self.get_log(logname)
        csv = "timestamp,reference,input,output\n"
        start_time = log["entries"][0]["timestamp"]
        print(start_time)
        for entry in log["entries"]:
            print(entry)
            entry = ",".join([str(entry["timestamp"] - start_time),
                              str(entry["reference"]),
                              str(entry["input"]),
                              str(entry["output"])])
            csv += entry + "\n"
        return csv




if __name__ == '__main__':
    server_url = "http://localhost:8000"
    print("Hello, World!")

    server = RustyBrew(server_url)

    print("Reference series:")
    reference_series = [{'duration': 15, 'temp': 55}, {'duration': 22, 'temp': 63}, {'duration': 120, 'temp': 64}]
    print(reference_series)

    print("Logs:")
    logs = server.get_list_of_logs()
    print(logs)
    for log in logs:
        print("Deleting: ", log)
        server.delete_log(log)
    logs = server.get_list_of_logs()
    print(logs)

    print("Resources:")
    resources = server.get_list_of_resources()
    print(resources)
    if not "Mock" in resources:
        print("ERROR: 'Mock' not a resource")
        quit()

    print("Reference series:")
    references = server.get_list_of_reference_series()
    print(references)
    if "test" in references:
        print("Deleteing")
        server.delete_reference_series("test")
    print("Posting")
    server.post_reference_series("test", reference_series)
    reference_series = server.get_list_of_reference_series()
    print(reference_series)
    if not "test" in reference_series:
        print("ERROR: reference series not posted")
        quit()

    print("Starting, and printing statuses:")
    server.start_controlling("Mock", "test")
    stop = False
    while not stop:
        status = "FAIL"
        try:
            status = server.get_current_values("Mock")
        except Exception:
            print("error")
        print(status)
        i = input("stop? ")
        if i == 'y':
            stop = True

    print("Logs:")
    logs = server.get_list_of_logs()
    print(logs)

    csv = server.get_log_as_csv(logs[0])

    with open("log.csv", 'w') as f:
        f.write(csv)
