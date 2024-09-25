class ColorIndex:

    def __init__(self, color1: str, color2: str):
        self.color1 = color1
        self.color2 = color2

    def __hash__(self) -> int:
        color = self.color1 + self.color2
        return hash(color)

    def __eq__(self, other) -> bool:
        return isinstance(other, ColorIndex) and self.color1 == other.color1 and self.color2 == other.color2
    
    def __str__(self) -> str:
        return f'({self.color1}, {self.color2})'