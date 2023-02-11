import requests
import json
import os
#import multiline
import base64
import re


# parsing URL file (with *only* GitHub links)
# with open(" .txt", "r") as file:
#    urls = file.readlines()
token = os.getenv("GITHUB_TOKEN")

# for url in urls:

#url = "https://github.com/cloudinary/cloudinary_npm"

url = "https://api.github.com/repos/cloudinary/cloudinary_npm"
headers = {'Authorization': f'Bearer {token}', 'Accept': 'application/json'}
response = requests.get(url, headers=headers)

response.raise_for_status()

if response.status_code == 200:
    pretty_data = json.loads(response.text)
    # with open("rest_data.txt", "w") as file:
    #     #file.write(pretty_data)
    #     for key, value in pretty_data.items(): 
    #         file.write('%s:%s\n' % (key, value))
    name = pretty_data["name"]
    hasLicense = pretty_data["license"]
    hasWiki = pretty_data["has_wiki"]
    hasDiscussions = pretty_data["has_discussions"]
    hasPages = pretty_data["has_pages"]


    newURL = "https://api.github.com/repos/cloudinary/cloudinary_npm/contents/"
    res2 = requests.get(newURL, headers=headers)
    pretty_content = json.loads(res2.text)
   
    names = []
    for i in range(len(pretty_content)): 
        names.append(pretty_content[i]["name"])

    #if "test" in names:
    if 'test'.casefold() in (name.casefold() for name in names):
        print("YIPPEE")
    if "README.md" in names:
        print("YIPPEE x 2")

    rmURL = "https://api.github.com/repos/cloudinary/cloudinary_npm/contents/README.md"
    res3 = requests.get(rmURL, headers=headers)
    pretty_readme = json.loads(res3.text)
    #print(pretty_readme["content"])

    rmbase64 = pretty_readme["content"]

    decoded = base64.b64decode(rmbase64)
    print(type(decoded))
    decodeStr = decoded.decode()
    print(decodeStr)
    if "Licence" in decodeStr:
        print("YIPPEE x 3")
        print(decodeStr.split("Licence",1)[1])

    licenses = {"Apache": 0, "Mit": 1, "GNU": 1}


else:
    print("Request failed with status code:", response.status_code)

