import requests
import json
import os


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

url = "https://github.com/cloudinary/cloudinary_npm"

my_headers = {'Authorization' : 'Bearer {token}'}
response = requests.get(url, headers=my_headers)

# response = requests.get(url, auth=('daloopy', my_var))
# repository_dict = response.json

print(response.json)