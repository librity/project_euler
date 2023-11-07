from typing import List


def fibonnaci(nth: int) -> List[int]:
    if nth <= 0:
        return []
    if nth == 1:
        return [1]
    if nth == 2:
        return [1, 2]

    nth = nth - 2
    fibs = [1, 2]
    for i in range(0, nth):
        prev = fibs[-1]
        prev_prev = fibs[-2]
        fibs.append(prev + prev_prev)

    return fibs


def fibonnaci_even_sum(up_to: int) -> int:
    fibs = fibonnaci(up_to)

    total = 0
    for fib in fibs:
        if fib % 2 == 0:
            total += fib

    return total


print(fibonnaci(10))
# print(fibonnaci(1000))
print(fibonnaci_even_sum(4000000))
