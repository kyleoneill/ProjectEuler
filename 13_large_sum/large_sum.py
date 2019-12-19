import time


def load_input(filename):
    output = None
    with open(filename, 'r') as file:
        content = file.read()
        output = content.split("\n")
    return output


def main():
    start = time.time()
    input_file = load_input('input.txt')
    lines = []
    for line in input_file:
        lines.append(int(line))
    total_sum = sum(lines)
    first_ten_digits = str(total_sum)[0:10]
    elapsed = (time.time() - start)
    print(f"Found solution in {elapsed} seconds.\nTotal Sum: {total_sum}\nSolution: {first_ten_digits}")


main()
