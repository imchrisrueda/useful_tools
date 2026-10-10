from manim import *


class GeneActivationScene(Scene):
    def construct(self):
        title = Text("Activación Génica", font_size=36, color=BLUE)
        self.play(Write(title))
        self.wait(0.5)

        circle = Circle(radius=1.5, color=GREEN)
        self.play(Transform(title, circle))
        self.wait(0.5)
