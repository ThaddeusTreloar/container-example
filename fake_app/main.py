from time import sleep

with open('log.log', 'r') as source:
    for line in source.readlines():
        sleep(0.5)
        print(line)