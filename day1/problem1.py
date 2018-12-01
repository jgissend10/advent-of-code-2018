def main(filename):
    """
    >>> main("test_files/test1.txt")
    3
    >>> main("test_files/test2.txt")
    0
    >>> main("test_files/test3.txt")
    -6
    """
    freq = 0

    with open(filename) as f:
        for line in f:
                freq += int(line)
    return freq


if __name__ == "__main__":
    import doctest
    doctest.testmod()

    input_file = "input.txt"
    ans = main(input_file)
    print(ans)
