import time

num_tests = 100000000
x = 7
str_val = "seven"
result = 0

# Measure time for if-else with int
start = time.time()
for _ in range(num_tests):
    if x == 1:
        result = 1
    elif x == 2:
        result = 2
    elif x == 3:
        result = 3
    elif x == 4:
        result = 4
    elif x == 5:
        result = 5
    elif x == 6:
        result = 6
    elif x == 7:
        result = 7
    elif x == 8:
        result = 8
    elif x == 9:
        result = 9
end = time.time()
print(f"if-else (int) time: {(end - start) * 1000} ms")

# Measure time for dict-based switch-case with int
start = time.time()
for _ in range(num_tests):
    result = {
        1: 1,
        2: 2,
        3: 3,
        4: 4,
        5: 5,
        6: 6,
        7: 7,
        8: 8,
        9: 9
    }.get(x, 0)
end = time.time()
print(f"switch-case (int) time: {(end - start) * 1000} ms")

# Measure time for if-else with string
start = time.time()
for _ in range(num_tests):
    if str_val == "one":
        result = 1
    elif str_val == "two":
        result = 2
    elif str_val == "three":
        result = 3
    elif str_val == "four":
        result = 4
    elif str_val == "five":
        result = 5
    elif str_val == "six":
        result = 6
    elif str_val == "seven":
        result = 7
    elif str_val == "eight":
        result = 8
    elif str_val == "nine":
        result = 9
end = time.time()
print(f"if-else (string) time: {(end - start) * 1000} ms")

# Measure time for dict-based switch-case with string
start = time.time()
for _ in range(num_tests):
    result = {
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9
    }.get(str_val, 0)
end = time.time()
print(f"switch-case (string) time: {(end - start) * 1000} ms")
