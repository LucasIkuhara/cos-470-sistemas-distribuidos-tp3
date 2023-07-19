from os import listdir
from re import sub
import pandas as pd
from datetime import datetime

# get all files with .clog extension
files = [f for f in listdir() if f.split(".")[-1] == "clog"]

# Create DF
df = pd.DataFrame({"N": [], "K": [], "R": [], "time (s)": []})


def parse_exec_time_from_log(filename: str) -> float:
    '''
    Compute time spent in seconds from logfile.
    '''

    with open(filename, "r") as fd:
        data = fd.readlines()
    
    start = data[0][:26]
    end = data[-1][:26]
    
    start = datetime.strptime(start, '%Y-%m-%d %H:%M:%S.%f')
    end = datetime.strptime(end, '%Y-%m-%d %H:%M:%S.%f')
    delta = end - start
    
    return delta.total_seconds()

# Parse all the data
for name in files:
    no_ext = name.split(".")[0]
    no_ext = sub(" *[A-z]*", "", no_ext)
    n, k, r = list(map(lambda x: int(x), no_ext.split("-")))

    time = parse_exec_time_from_log(name)

    df = df.append({"N": n, "K": k, "R": r, "time (s)": time}, ignore_index=True)

df.to_csv("results.csv", index=False)
