tester = 1

def test(*args, **kwargs):
    print(args)
    print(kwargs)

class Test:
    def __init__(self, *args, **kwargs):
        print(args)
        print(kwargs)
