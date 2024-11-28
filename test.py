cnt = 0
while 1:
    cnt = (cnt + 1)%100
    print(f"\033[1A{cnt}", end="\n")
