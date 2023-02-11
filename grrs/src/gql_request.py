from gql import *
# from gql import gql, Client
from gql import Client, gql
from gql.transport.requests import RequestsHTTPTransport
import os

token = os.getenv("GITHUB_TOKEN")

headers = {
    "Authorization": "Token {}".format(token)
}

# Use the RequestsHTTPTransport class to send the GraphQL query with the headers
transport = RequestsHTTPTransport(
    url="https://api.github.com/graphql",
    headers=headers,
    use_json=True,
)

# Create a client using the transport
client = Client(transport=transport, fetch_schema_from_transport=True)


query_string = """
{ 
  repository(owner:"lodash", name:"lodash") { 
    issues {
      totalCount
    }
    open: issues(states:OPEN) {
      totalCount
    }
    closed: issues(states:CLOSED) {
      totalCount
    }
  }
}
"""

# Provide a GraphQL query
query = gql(query_string)


# Execute the query on the transport
result = client.execute(query)

print(result)





