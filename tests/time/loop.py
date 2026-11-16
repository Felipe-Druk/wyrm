i = 0
acc = 0
while i < 1000000:
    acc = (acc + i * 3) % 1000007
    i += 1
print(acc)