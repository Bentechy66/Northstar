import random
from collections import Counter
import statistics

import numpy as np
from scipy.interpolate import interp1d
import matplotlib.pyplot as plt

N_ITER = 1000000
BASE_ODDS = 0.015
HOURS_RUNNING = 250
MINS_PER_RUN = 40
TIME_PER_RUN = MINS_PER_RUN / 60

def new():
    iterations = 0
    math = []

    while True:
        successes = 0
        odds = BASE_ODDS
        i = 0

        for _ in range(int(HOURS_RUNNING / TIME_PER_RUN)):
            i += 1
            if random.random() <= odds:
                successes += 1
                odds = BASE_ODDS
            
            odds *= 1.02
        
        math.append(successes)
        
        if iterations % 100000 == 0:
            print(f"Iter {iterations}: avg {sum(math)/len(math)} max {max(math)} min {min(math)}")
        
        if iterations == N_ITER:
            break

        iterations += 1

    return Counter(math)

def old():
    iterations = 0
    math = []

    while True:
        successes = 0
        odds = BASE_ODDS

        for _ in range(int(HOURS_RUNNING / TIME_PER_RUN)):
            if random.random() <= odds:
                successes += 1
        
        math.append(successes)
        
        if iterations % 100000 == 0:
            print(f"Iter {iterations}: avg {sum(math)/len(math)} max {max(math)} min {min(math)}")
        
        if iterations == N_ITER:
            break

        iterations += 1

    return Counter(math)

def plot(c, col):
    z = {}

    for key in c:
        z[key] = (c[key] / len(list(c.elements()))) * 100

    z = {key: z[key] for key in sorted(z.keys())}

    x = np.array(list(z.keys()))
    y = np.array(list(z.values()))

    cubic_interploation_model = interp1d(x, y, kind = "cubic")

    X_=np.linspace(x.min(), x.max(), 500)
    Y_=cubic_interploation_model(X_)

    mean = sum(list(c.elements()))/len(list(c.elements()))

    plt.axvline(x=mean, linestyle="--", label="mean", color=col)

    plt.plot(X_, Y_)

# ===============

c = new()
plot(c, "blue")
c = old()
plot(c, "orange")

plt.title(f"Average mythic count per {HOURS_RUNNING} hours played (base chance {BASE_ODDS*100}%, {MINS_PER_RUN}m per run)")
plt.xlabel("Number Of Mythics")
plt.ylabel("Probability (%)")
plt.show()