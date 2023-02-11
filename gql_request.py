from gql import *
import json
from gql import Client, gql
from gql.transport.requests import RequestsHTTPTransport
import os
import re


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
  client = Client(transport=transport, fetch_schema_from_transport=True)

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
  query = gql(response_query)

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

  query = gql(bus_query)  
  bus_result = client.execute(query) 

  #format data
  data = {
    "issues": {
      "open": response_result["repository"]["open"]["totalCount"],
      "closed": response_result["repository"]["closed"]["totalCount"],
      "total": response_result["repository"]["issues"]["totalCount"]
    },
    "total_commits": bus_result["repository"]["object"]["history"]["totalCount"]
  }

  return data

def getOwnerRepo(url):
  #url = "https://github.com/cloudinary/cloudinary_npm"
  parts = re.split("/", url)
  owner = parts[-2]
  repo = parts[-1]
  repo = repo[:-1]

  print("repo", repo)
  print("owner", owner)


  return owner, repo

def getData():
  filenames = []
  with open("URLs.txt", "r") as inputfile:
    urls = inputfile.readlines()

  for url in urls:
    owner,repo = getOwnerRepo(url)
    gqldata = getGqlData(owner, repo)
    #restdata = getRestData(owner, repo)

    data = gqldata
    #data["hasLicense"] = restdata["hasLicense"]
    #data["hasWiki"] = restdata["hasWiki"]
    #data["hasPages"] = restdata["hasPages"]
    #data["hasDiscussions"] = restdata["hasDiscussions"]

    #create filename
    filename = owner+"_data.txt"

    with open(filename, "w") as outputfile:
      outputfile.write(str(data))

    filenames.append(filename)

  return filenames



filename = getData()

