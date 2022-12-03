import logging

logging.basicConfig(level="INFO")


def parse(calorie_list: str) -> list[list[int]]:
    calories = [
        [int(item) for item in elf.splitlines()] for elf in calorie_list.split("\n\n")
    ]
    logging.debug(f"{calories=}")
    return calories


def max_calories(calories: list[list[int]]) -> int:
    return max(sum(c) for c in calories)


def max_3_calories(calories: list[list[int]]) -> int:
    return sum(sorted([sum(c) for c in calories], reverse=True)[:3])


if __name__ == "__main__":
    import argparse
    import sys

    parser = argparse.ArgumentParser()
    parser.add_argument(
        "infile", nargs="?", type=argparse.FileType("r"), default=sys.stdin
    )
    args = parser.parse_args()
    calories = parse(args.infile.read())
    print(max_calories(calories))
    print(max_3_calories(calories))
