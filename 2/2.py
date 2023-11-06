def fibonnaci(nth: int) -> int:
    if nth < 2:
        return 1

    return fibonnaci(nth - 1) + fibonnaci(nth - 2)


def fibonnaci_even_sum(up_to: int) -> int:
    total = 0

    for i in range(1, up_to):
        print(".", end="", flush=True)
        fib = fibonnaci(i)
        if fib % 2 == 0:
            total += fib

    return fib


print(fibonnaci(10))
print(fibonnaci_even_sum(4000000))
