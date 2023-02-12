## What is this project?
This project is a CLI written in Rust and Python that, with a .txt file of package URLs, 
outputs a sorted list of package URLs along with their scores for bus factor, responsiveness, correctness, ramp-up time, and license compatibility to LGPL v2.1. The CLI is also made with open-source packages and libraries.

This is Part 1 of the semester project for ECE461 (Spring 2023) at Purdue University.

## The team
Jason Jones  
Anonya Mitra  
Dalilah Vaquera  
Wahab William Akanbi  


## To run our project
navigate into the grrs folder and run 
```

    ./run _____

```
where _____ is one of the following in this order: install, build, URL_FILE, test  
URL_FILE is the absolute location of a file consisting of an ASCII-encoded newline-delimited set of URLs  


## Succinct description of data flow
The main function (created in rust) first parses the URLs.txt file line by line and initializes a Packages object, setting the URL attribute to the URL read from the file and the other metric score attributes to -1. Next, the npm URLs are converted to GitHub URLs, which prepares them to be called by the GitHub API.

The function then uses PyO3 to call the functions written in api.py, which make calls to both the GitHub REST API and GitHub GraphQL API, parses through the data returned from the REST API, and outputs the data we selected back out to the main function. We also output 2 out of the 5 metric scores needed since parsing and validation were easier and quicker in Python.

We then call our metric calculation functions to get the rest of the scores and output a sorted list of package URLs, their total score, and their metric scores.
