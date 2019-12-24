import math

def main(x_len, y_len):
    # Formula = ((x+y)! / x!) / y!
    number_of_movements = x_len + y_len
    combinations_with_redundancies = math.factorial(number_of_movements) / math.factorial(x_len)
    answer = combinations_with_redundancies / math.factorial(y_len)
    print(f"Answer: {answer}")
    return 0

main(20, 20)
