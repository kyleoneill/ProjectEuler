import math

def main(base_num, exponent):
    large_num = base_num ** exponent
    num_as_list = [int(d) for d in str(large_num)]
    power_digit_sum = 0
    for digit in num_as_list:
        power_digit_sum += digit
    print(f"Answer is: {power_digit_sum}")
    return 0

main(2, 1000)
