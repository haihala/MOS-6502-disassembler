import sys
import json
from math import ceil

import httpx

data = []
with open(sys.argv[1], "rb") as f:
    while b := f.read(1):
        data.append(ord(b))

print("Bytes:")
row_len = 8
for row in range(ceil(len(data)/row_len)):
    start = row*row_len
    end = min(start+row_len, len(data))
    print(" ".join("{:02X}".format(i) for i in data[start:end]))

print()

r = httpx.post(
    "http://127.0.0.1:9999",
    headers={
        "Content-Type": "application/json"
    },
    data=json.dumps({"data": data})
)

print(r.status_code)
for line in r.json()["disassembly"]:
    print(line)
