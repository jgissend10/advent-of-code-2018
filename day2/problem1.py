from collections import Counter


def main(filename):
    """
    >>> main("test_files/test1.txt")
    12
    """

    with open(filename) as f:
        count_2 = 0
        count_3 = 0

        counters = [Counter(i) for i in f.readlines()]
        for counter in counters:
            if 2 in counter.values():
                count_2 += 1
            if 3 in counter.values():
                count_3 += 1
        checksum = count_2 * count_3
        return checksum


if __name__ == "__main__":
    import doctest
    doctest.testmod()

    input_file = "input.txt"
    ans = main(input_file)
    print(ans)
