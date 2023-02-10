import requests
import os


# parsing URL file (with *only* GitHub links)
with open(" .txt", "r") as file:
   urls = file.readlines()


my_var = os.getenv("GITHUB_TOKEN")
if my_var:
    # Use the value of MY_VAR
    print("MY_VAR is set to:", my_var)
else:
    # MY_VAR is not set
    print("MY_VAR is not set")


for url in urls:

    #get our content
    response = requests.get(url, auth=('daloopy', my_var))

    repository_dict = response.json()

    #write to txt file -> go to rust for our classes and calculations
    with open("data.txt", "w") as file:
       file.writelines(repository_dict)