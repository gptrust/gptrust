import gptrustpy

def main():
    inline = input("¯\_(ツ)_/¯ ? ")
    outline = gptrustpy.chat_complete(inline)
    print("<|°_°|>", outline)
    return 0

if __name__ == "__main__":
    exit(main())
