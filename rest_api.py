import requests
import json
import os
#import multiline


# parsing URL file (with *only* GitHub links)
# with open(" .txt", "r") as file:
#    urls = file.readlines()
token = os.getenv("GITHUB_TOKEN")

# for url in urls:

#     #get our content
#     response = requests.get(url, auth=('daloopy', my_var))

#     repository_dict = response.json()

#     #write to txt file -> go to rust for our classes and calculations
#     with open("data.txt", "w") as file:
#        file.writelines(repository_dict)

#url = "https://github.com/cloudinary/cloudinary_npm"

url = "https://api.github.com/repos/cloudinary/cloudinary_npm"
headers = {'Authorization': f'Bearer {token}', 'Accept': 'application/json'}
response = requests.get(url, headers=headers)

response.raise_for_status()

if response.status_code == 200:
    pretty_data = json.loads(response.text)
    with open("rest_data.txt", "w") as file:
        #file.write(pretty_data)
        for key, value in pretty_data.items(): 
            file.write('%s:%s\n' % (key, value))
else:
    print("Request failed with status code:", response.status_code)

