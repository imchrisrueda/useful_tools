"""
Backpropagation Neural Network Animation Scene for Intelligent Presentation Studio (IPS).
Demonstrates:
- 1 Hidden Layer Neural Network (2 inputs, 3 hidden, 1 output)
- Forward pass activation
- Error calculation at output
- Backward gradient propagation (Chain Rule)
- Gradual weight updates across iterations minimizing loss
"""

from manim import *


class BackpropagationScene(Scene):
    def construct(self):
        # Background and global settings
        self.camera.background_color = "#0b0f19"

        # 1. Title Banner
        title = Text("Backpropagation: Red Neuronal con 1 Capa Oculta", font_size=28, color=WHITE)
        title.to_edge(UP, buff=0.35)
        self.play(Write(title), run_time=1.0)

        # 2. Network Layout coordinates
        # Layer 1: Input (x1, x2)
        in_pos = [np.array([-4.0, 1.3, 0]), np.array([-4.0, -1.3, 0])]
        # Layer 2: Hidden (h1, h2, h3)
        hid_pos = [np.array([0.0, 2.0, 0]), np.array([0.0, 0.0, 0]), np.array([0.0, -2.0, 0])]
        # Layer 3: Output (y)
        out_pos = [np.array([4.0, 0.0, 0])]

        # Neuron Visuals
        input_neurons = VGroup(*[Circle(radius=0.45, color=BLUE_B, fill_color="#1e293b", fill_opacity=1) for _ in in_pos])
        for n, p in zip(input_neurons, in_pos):
            n.move_to(p)

        hidden_neurons = VGroup(*[Circle(radius=0.45, color=TEAL_C, fill_color="#1e293b", fill_opacity=1) for _ in hid_pos])
        for n, p in zip(hidden_neurons, hid_pos):
            n.move_to(p)

        output_neurons = VGroup(*[Circle(radius=0.45, color=PURPLE_B, fill_color="#1e293b", fill_opacity=1) for _ in out_pos])
        for n, p in zip(output_neurons, out_pos):
            n.move_to(p)

        # Labels for neurons
        in_labels = VGroup(
            Text("x1", font_size=18, color=WHITE).move_to(in_pos[0]),
            Text("x2", font_size=18, color=WHITE).move_to(in_pos[1])
        )
        hid_labels = VGroup(
            Text("h1", font_size=18, color=WHITE).move_to(hid_pos[0]),
            Text("h2", font_size=18, color=WHITE).move_to(hid_pos[1]),
            Text("h3", font_size=18, color=WHITE).move_to(hid_pos[2])
        )
        out_labels = VGroup(
            Text("y^", font_size=18, color=WHITE).move_to(out_pos[0])
        )

        # Layer headers
        header_in = Text("Entrada (2)", font_size=16, color=BLUE_B).next_to(in_pos[0], UP, buff=0.6)
        header_hid = Text("Oculta (3)", font_size=16, color=TEAL_C).next_to(hid_pos[0], UP, buff=0.6)
        header_out = Text("Salida (1)", font_size=16, color=PURPLE_B).next_to(out_pos[0], UP, buff=0.8)

        # Synapses / Connections
        in_to_hid_edges = VGroup()
        for i_p in in_pos:
            for h_p in hid_pos:
                line = Line(i_p, h_p, stroke_width=2.5, color=GREY_B, buff=0.45)
                in_to_hid_edges.add(line)

        hid_to_out_edges = VGroup()
        for h_p in hid_pos:
            line = Line(h_p, out_pos[0], stroke_width=2.5, color=GREY_B, buff=0.45)
            hid_to_out_edges.add(line)

        # Status text at bottom
        status_box = Rectangle(width=11, height=0.9, color="#334155", fill_color="#0f172a", fill_opacity=0.9)
        status_box.to_edge(DOWN, buff=0.25)
        status_text = Text("Topología: 2 Entradas -> 1 Capa Oculta (3) -> 1 Salida", font_size=17, color=LIGHT_GREY)
        status_text.move_to(status_box)

        # Create Network on Screen
        self.play(
            FadeIn(header_in), FadeIn(header_hid), FadeIn(header_out),
            Create(in_to_hid_edges), Create(hid_to_out_edges),
            Create(input_neurons), Create(hidden_neurons), Create(output_neurons),
            Write(in_labels), Write(hid_labels), Write(out_labels),
            FadeIn(status_box), Write(status_text),
            run_time=1.5
        )
        self.wait(0.5)

        # ==========================================
        # FASE 1: FORWARD PASS
        # ==========================================
        st1 = Text("1. Forward Pass: Propagación de activaciones x -> h -> y^", font_size=17, color=YELLOW_B)
        st1.move_to(status_box)
        self.play(Transform(status_text, st1), run_time=0.6)

        # Activación entradas -> oculta
        self.play(
            input_neurons.animate.set_fill(BLUE_E, opacity=1),
            in_to_hid_edges.animate.set_color(BLUE_C).set_stroke(width=3.5),
            run_time=0.8
        )
        self.play(
            hidden_neurons.animate.set_fill(TEAL_E, opacity=1),
            hid_to_out_edges.animate.set_color(TEAL_C).set_stroke(width=3.5),
            run_time=0.8
        )
        self.play(
            output_neurons.animate.set_fill(YELLOW_E, opacity=1),
            run_time=0.6
        )

        # ==========================================
        # FASE 2: CÁLCULO DEL ERROR EN SALIDA
        # ==========================================
        loss_val = Text("Predicción y^ = 0.85 | Real y = 0.20 | Error L = 0.211", font_size=16, color=RED_B)
        loss_val.next_to(output_neurons, DOWN, buff=0.8)

        st2 = Text("2. Evaluación de Error: Discrepancia detectada en la salida", font_size=17, color=RED_A)
        st2.move_to(status_box)
        self.play(
            Transform(status_text, st2),
            output_neurons.animate.set_fill(RED_E, opacity=1).set_color(RED),
            FadeIn(loss_val),
            run_time=0.8
        )
        self.wait(0.6)

        # ==========================================
        # FASE 3: BACKPROPAGATION (GRADIENTES)
        # ==========================================
        st3 = Text("3. Backpropagation: Regla de la cadena retropropagando dL/dw", font_size=17, color=ORANGE)
        st3.move_to(status_box)
        self.play(Transform(status_text, st3), run_time=0.6)

        # Pulso inverso de salida a capa oculta
        self.play(
            hid_to_out_edges.animate.set_color(ORANGE).set_stroke(width=4.5),
            hidden_neurons.animate.set_fill("#7c2d12", opacity=1).set_color(ORANGE),
            run_time=1.0
        )
        # Pulso inverso de oculta a entrada
        self.play(
            in_to_hid_edges.animate.set_color(RED_B).set_stroke(width=4.5),
            input_neurons.animate.set_fill("#831843", opacity=1),
            run_time=1.0
        )
        self.wait(0.5)

        # ==========================================
        # FASE 4: AJUSTE GRADUAL DE PESOS (3 ÉPOCAS)
        # ==========================================
        # Época 1
        st_ep1 = Text("4. Ajuste gradual: Época 1 -> w = w - eta * grad (Error: 0.211 -> 0.115)", font_size=17, color=GOLD_B)
        st_ep1.move_to(status_box)
        loss_val_ep1 = Text("Predicción y^ = 0.61 | Real y = 0.20 | Error L = 0.084", font_size=16, color=GOLD_B)
        loss_val_ep1.move_to(loss_val)

        self.play(
            Transform(status_text, st_ep1),
            Transform(loss_val, loss_val_ep1),
            in_to_hid_edges.animate.set_color(GOLD_C).set_stroke(width=3.2),
            hid_to_out_edges.animate.set_color(GOLD_C).set_stroke(width=3.2),
            output_neurons.animate.set_fill(GOLD_E, opacity=1).set_color(GOLD_B),
            run_time=1.0
        )
        self.wait(0.5)

        # Época 2
        st_ep2 = Text("4. Ajuste gradual: Época 2 -> Conexiones calibrándose (Error: 0.084 -> 0.022)", font_size=17, color=GREEN_B)
        st_ep2.move_to(status_box)
        loss_val_ep2 = Text("Predicción y^ = 0.35 | Real y = 0.20 | Error L = 0.011", font_size=16, color=GREEN_B)
        loss_val_ep2.move_to(loss_val)

        self.play(
            Transform(status_text, st_ep2),
            Transform(loss_val, loss_val_ep2),
            in_to_hid_edges.animate.set_color(TEAL_B).set_stroke(width=2.8),
            hid_to_out_edges.animate.set_color(TEAL_B).set_stroke(width=2.8),
            hidden_neurons.animate.set_fill(TEAL_E, opacity=1).set_color(TEAL_B),
            run_time=1.0
        )
        self.wait(0.5)

        # Época 3 (Convergencia)
        st_conv = Text("Convergencia completada: Pesos calibrados (y^ = 0.205 ~ y = 0.20)", font_size=17, color=GREEN_A)
        st_conv.move_to(status_box)
        loss_val_conv = Text("Predicción y^ = 0.205 | Real y = 0.20 | Error L = 0.0001", font_size=16, color=GREEN_A)
        loss_val_conv.move_to(loss_val)

        self.play(
            Transform(status_text, st_conv),
            Transform(loss_val, loss_val_conv),
            in_to_hid_edges.animate.set_color(GREEN_C).set_stroke(width=2.5),
            hid_to_out_edges.animate.set_color(GREEN_C).set_stroke(width=2.5),
            output_neurons.animate.set_fill(GREEN_E, opacity=1).set_color(GREEN_A),
            input_neurons.animate.set_fill("#1e293b", opacity=1).set_color(BLUE_B),
            hidden_neurons.animate.set_fill("#1e293b", opacity=1).set_color(TEAL_C),
            run_time=1.2
        )
        self.wait(1.0)
