import gql
import json
from gql.transport.requests import RequestsHTTPTransport
import os
import re
import requests
import base64

def getRestData(owner, repo):

  token = os.getenv("GITHUB_TOKEN") #authentication 

  #making REST request
  url = "https://api.github.com/repos/{}/{}".format(owner, repo)
  headers = {'Authorization': f'Bearer {token}', 'Accept': 'application/json'}
  response = requests.get(url, headers=headers)

  response.raise_for_status()
  if response.status_code == 200:
    pretty_data = json.loads(response.text)

    #making second request for repository content
    contentURL = "https://api.github.com/repos/{}/{}/contents/".format(owner, repo)
    content_resp = requests.get(contentURL, headers=headers)
    content_resp.raise_for_status()
    if content_resp.status_code == 200:
      pretty_content = json.loads(content_resp.text)

      #get names of all files/directories
      names = [] 
      for i in range(len(pretty_content)): 
        names.append(pretty_content[i]["name"])

      test_score = 0.0 
      hasREADME = False   
      #if testing dir/file(s) present, set to 1
      if 'test'.casefold() in (name.casefold() for name in names):
        test_score = 1.0
      # if README in repo  
      if "README.md" in names: 
        hasREADME = True
      # getting more info (this plus hasREADME = ramp-up data)
      hasWiki = pretty_data["has_wiki"]
      hasDiscussions = pretty_data["has_discussions"]
      hasPages = pretty_data["has_pages"]
      
      # checking if license info available through REST API
      license_score = 0.0
      hasLicense = pretty_data["license"]
      if hasLicense == "False":
        # if not through REST, then present in README (hopefully)
        # making third request for README.md
        RMurl = "https://api.github.com/repos/{}/{}/contents/README.md".format(owner, repo)
        RM_resp = requests.get(RMurl, headers=headers)
        RM_resp.raise_for_status()
        if RM_resp.status_code == 200:
          pretty_readme = json.loads(RM_resp.text)
          rmbase64 = pretty_readme["content"] # the text in README, base64 encoded

          #decode base64 and make into string
          decoded = base64.b64decode(rmbase64)
          decodeStr = decoded.decode()
          # all popular licenses and their compatibility score with LGPL 2.1 as defined 
          licenses = {"Apache": 0.0, "MIT": 1.0, "GNU": 1.0, "GPL": 1.0, "LGPL": 1.0, "MPL": 1.0, "Eclipse Public License": 0.0, "BSD": 1.0, "CDDL": 1.0}
          license_score = 0.5

          #license in README or not mentioned/available in repo
          # license compatible = 1, lincese exists but not compatible = 0.5, license doesn't exist = 0
          #if "Licence" in decodeStr or "License" in decodeStr:
          if 'Licence'.casefold() in decodeStr.casefold():
            licenseStr = decodeStr.split("Licence".casefold(),1)[1] 
            # check license in dictionary and update score
            for key, val in licenses.items():
                if key in licenseStr:
                  license_score = val
          elif 'License'.casefold in decodeStr.casefold():
            licenseStr = decodeStr.split("License".casefold(),1)[1] 
            for key, val in licenses.items():
                if key in licenseStr:
                  license_score = val
          else:
            print("Request failed with status code:", response.status_code)
      else:
        license_score = 1.0
    else:
      print("Request failed with status code:", response.status_code)
    
    # making fourth request for contributors and their commits/contributions
    contributeURL = "https://api.github.com/repos/{}/{}/contributors?per_page=10".format(owner, repo)
    contributors_resp = requests.get(contributeURL, headers=headers)
    contributors_resp.raise_for_status()
    if contributors_resp.status_code == 200:
      pretty_people = json.loads(contributors_resp.text)
      commits_sum = 0 # sum of all contributions/commits of person
      for i in range(len(pretty_people)):
        commits_sum += pretty_people[i]["contributions"]
    else:
      print("Request failed with status code:", response.status_code)
  else:
    print("Request failed with status code:", response.status_code)

  return test_score, license_score, hasWiki, hasDiscussions, hasPages, hasREADME, commits_sum

 

def getGqlData(owner, repo):
  token = os.getenv("GITHUB_TOKEN")   # get personal github api token
  headers = {"Authorization": "Token {}".format(token)}

  # Use the RequestsHTTPTransport class to send the GraphQL query with the headers
  transport = RequestsHTTPTransport(
    url="https://api.github.com/graphql",
    headers=headers,
    use_json=True,
  )

  # Create a client using the transport
  client = gql.Client(transport=transport, fetch_schema_from_transport=True)

### RESPONSE QUERY
  # create query
  response_query = """
  {{ 
  repository(owner:"{}", name:"{}") {{ 
    name
    issues {{
      totalCount
    }}
    open: issues(states:OPEN) {{
      totalCount
    }}
    closed: issues(states:CLOSED) {{
      totalCount
    }}
  }}
  }}
""".format(owner, repo)

  # Provide a GraphQL query
  query = gql.gql(response_query)

  # Execute the query on the transport
  response_result = client.execute(query) 

### BUS QUERY
  bus_query ="""
  {{
  repository(owner:"{}", name:"{}") {{
    object(expression:"master") {{
      ... on Commit {{
        history {{
          totalCount
        }}
      }}
    }}
  }}
}}
""".format(owner, repo)

  query = gql.gql(bus_query)
  bus_result = client.execute(query)

  #format data
  data = {
    "open_issues": response_result["repository"]["open"]["totalCount"],
    "closed_issues": response_result["repository"]["closed"]["totalCount"],
    "total_commits": bus_result["repository"]["object"]["history"]["totalCount"]
  }

  return data

def getOwnerRepo(url):
  parts = re.split("/", url)
  owner = parts[0]
  repo = parts[1]
  return owner, repo

def getData(owner_repo):
    owner,repo = getOwnerRepo(owner_repo)
    gqldata = getGqlData(owner, repo)
    test_score, license_score, hasWiki, hasDiscussions, hasPages, hasREADME, busTeamCommits = getRestData(owner, repo)

    data = gqldata
    data["has_readme"] = hasREADME
    data["has_wiki"] = hasWiki
    data["has_pages"] = hasPages
    data["has_discussions"] = hasDiscussions
    data["bus_commits"] = busTeamCommits
    data["correctness_score"] = test_score
    data["license_score"] = license_score
    return json.dumps(data)
