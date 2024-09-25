from pynput.mouse import Button, Controller

def main():
    mouse = Controller()
    mouse.position = (140, 192)
    mouse.click(Button.right)

if __name__ == "__main__":
    main()
