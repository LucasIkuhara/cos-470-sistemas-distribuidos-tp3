import subprocess as sp
import pandas as pd
import numpy as np


# Params
n_range = [2, 4, 8, 16, 32, 64, 128]
repetitions = 3
k_time = 0

# Create DF
for n in n_range:

    print(f"Starting tests with N={n}, K={k_time}, R={repetitions}.\n")

    try:
        out = sp.run(["./target/debug/client", f"-a{k_time}", f"-r{repetitions}", f"-l n{n}-k{k_time}-r{repetitions}.clog"])
    except sp.CalledProcessError:
        print(f"Failed execution for N={n}, K={k_time}, R={repetitions}. Ignoring..")
        continue
