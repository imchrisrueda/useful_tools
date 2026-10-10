"""
IPS Scientific Animation Module (Phase 3).
Generates multi-segment mathematical and scientific explanations
with strict deterministic fallback posters for static export.
"""

import hashlib
import json
from pathlib import Path
from typing import Dict, List, Any


class AnimationSegment:
    def __init__(self, segment_id: str, title: str, duration_sec: float, poster_path: str):
        self.segment_id = segment_id
        self.title = title
        self.duration_sec = duration_sec
        self.poster_path = poster_path

    def to_dict(self) -> Dict[str, Any]:
        return {
            "segmentId": self.segment_id,
            "title": self.title,
            "durationSec": self.duration_sec,
            "posterPath": self.poster_path,
        }


class AnimationGenerator:
    """Deterministic scientific animation provider with SVG segment rendering."""

    def __init__(self, output_dir: Path):
        self.output_dir = output_dir
        self.output_dir.mkdir(parents=True, exist_ok=True)

    def generate_gene_expression_animation(
        self, scene_id: str, parameters: Dict[str, Any]
    ) -> Dict[str, Any]:
        """
        Generates a 3-step sequential scientific animation (segment 1: basal, segment 2: activation, segment 3: response)
        complete with control manifest and static SVG fallback for PDF/web export.
        """
        segments: List[AnimationSegment] = []

        # Step 1: Basal state
        s1_file = self.output_dir / f"{scene_id}_step1_basal.svg"
        s1_content = self._render_svg_step("1. Estado Basal", "#64748b", "Nivel basal de transcripción")
        s1_file.write_text(s1_content, encoding="utf-8")
        segments.append(AnimationSegment("step-1-basal", "Estado Basal", 3.0, str(s1_file.name)))

        # Step 2: Factor binding & activation
        s2_file = self.output_dir / f"{scene_id}_step2_activation.svg"
        s2_content = self._render_svg_step("2. Activación Molecular", "#3b82f6", "Unión de factor de transcripción")
        s2_file.write_text(s2_content, encoding="utf-8")
        segments.append(AnimationSegment("step-2-activation", "Activación Molecular", 4.5, str(s2_file.name)))

        # Step 3: High-yield expression response
        s3_file = self.output_dir / f"{scene_id}_step3_response.svg"
        s3_content = self._render_svg_step("3. Respuesta Terapéutica", "#10b981", "Síntesis máxima de ARN mensajero")
        s3_file.write_text(s3_content, encoding="utf-8")
        segments.append(AnimationSegment("step-3-response", "Respuesta Terapéutica", 5.0, str(s3_file.name)))

        # Static fallback poster for export (state 3)
        fallback_file = self.output_dir / f"{scene_id}_static_fallback.svg"
        fallback_file.write_text(s3_content, encoding="utf-8")

        manifest = {
            "sceneId": scene_id,
            "provider": "ips-scientific-animator",
            "parameters": parameters,
            "segments": [s.to_dict() for s in segments],
            "controls": {
                "keyboard": ["Space (play/pause)", "Left (previous step)", "Right (next step)", "R (reset)"],
                "supportsReverse": True,
                "reducedMotionSupported": True,
            },
            "staticFallbackRef": str(fallback_file.name),
            "manifestHash": hashlib.sha256(json.dumps(parameters, sort_keys=True).encode()).hexdigest(),
        }

        manifest_file = self.output_dir / f"{scene_id}_animation_manifest.json"
        manifest_file.write_text(json.dumps(manifest, indent=2), encoding="utf-8")

        return manifest

    def _render_svg_step(self, title: str, accent_color: str, description: str) -> str:
        return f"""<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 450" width="800" height="450">
  <rect width="100%" height="100%" fill="#0f172a" />
  <text x="400" y="80" font-family="sans-serif" font-size="24" font-weight="bold" fill="#ffffff" text-anchor="middle">{title}</text>
  <circle cx="400" cy="225" r="90" fill="{accent_color}" opacity="0.8" />
  <text x="400" y="360" font-family="sans-serif" font-size="16" fill="#94a3b8" text-anchor="middle">{description}</text>
</svg>"""
