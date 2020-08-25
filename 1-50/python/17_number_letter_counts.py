import math

below_twenty = [
  0,
  len("one"),
  len("two"),
  len("three"),
  len("four"),
  len("five"),
  len("six"),
  len("seven"),
  len("eight"),
  len("nine"),
  len("ten"),
  len("eleven"),
  len("twelve"),
  len("thirteen"),
  len("fourteen"),
  len("fifteen"),
  len("sixteen"),
  len("seventeen"),
  len("eighteen"),
  len("nineteen")
]

tens = [
    len("twenty"),
    len("thirty"),
    len("forty"),
    len("fifty"),
    len("sixty"),
    len("seventy"),
    len("eighty"),
    len("ninety")
]

def below_100(n):
  if n < 20:
    return below_twenty[n]
  return tens[int(math.floor(n / 10) - 2) or 0] + below_twenty[n % 10]

def numberLength(n):
    if n < 100:
        return below_100(n)
    res = 0
    hundreds = math.floor(n / 100) % 10
    thousands = math.floor(n / 1000)
    below_hundred = n % 100
    if n > 999:
        res += below_100(thousands) + len("thousand")
    if hundreds is not 0:
        res += below_twenty[hundreds] + len("hundred")
    if below_hundred is not 0:
        res += len("and") + below_100(below_hundred)
    return res

def main(num):
    total_sum = 0
    for num in range(1, num+1):
        total_sum += numberLength(num)
    print(f"Answer is: {total_sum}")
    return 0

main(1000)
