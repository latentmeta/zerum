def load():
    try:
        return open("data.txt").read()
    except ValueError:
        pass
