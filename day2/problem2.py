import itertools
from collections import Counter


def get_common_letters(stringa, stringb):
    common_string = ""
    for a, b in zip(stringa, stringb):
        if a == b:
            common_string += a
    return common_string


def compare_counters(counter_a, counter_b):
    diff = set(counter_a.items()).symmetric_difference(set(counter_b.items()))
    if len(diff) == 2 or len(diff) == 3:
        return True
    return False


def main(filename):
    """
    >>> main("test_files/test1.txt")
    'abcde'
    >>> main("test_files/test2.txt")
    'fgij'
    """

    with open(filename) as f:

        words = [(i.strip(), Counter(i.strip())) for i in f.readlines()]
        word_product = itertools.combinations(words, 2)
        for i, j in word_product:
            if i[0] == j[0]:
                continue
            if not compare_counters(i[1], j[1]):
                continue
            common = get_common_letters(i[0], j[0])
            if len(common) == len(i[0]) - 1:
                return common


if __name__ == "__main__":
    import doctest
    doctest.testmod()

    input_file = "input.txt"
    ans = main(input_file)
    print(ans)
