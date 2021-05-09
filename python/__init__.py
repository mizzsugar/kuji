import random


def main():
    names: Final = input().split(' ')
    print(random.choice(names))


if __name__ == '__main__':
    main()
