import itertools


def main(filename):
    """
    >>> main("test_files/test4.txt")
    0
    >>> main("test_files/test5.txt")
    10
    >>> main("test_files/test6.txt")
    5
    >>> main("test_files/test7.txt")
    14
    """
    freq = 0
    freqs = set([freq])

    with open(filename) as f:
        # ints = [int(i) for in in f.readlines()]
        ints = map(int, f.readlines())
        for number in itertools.cycle(ints):
            freq += number
            if freq in freqs:
                return freq
            else:
                freqs.add(freq)


if __name__ == "__main__":
    import doctest
    doctest.testmod()

    input_file = 'input.txt'
    ans = main(input_file)
    print(ans)
