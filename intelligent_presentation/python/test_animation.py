import tempfile
from pathlib import Path
from animation_generator import AnimationGenerator


def test_animation_generation():
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp_path = Path(tmpdir)
        generator = AnimationGenerator(tmp_path)

        manifest = generator.generate_gene_expression_animation(
            scene_id="gene-activation", parameters={"concentration_uM": 10.5, "duration_sec": 48}
        )

        assert manifest["sceneId"] == "gene-activation"
        assert len(manifest["segments"]) == 3
        assert manifest["controls"]["supportsReverse"] is True
        assert manifest["controls"]["reducedMotionSupported"] is True

        # Check generated files
        assert (tmp_path / "gene-activation_step1_basal.svg").exists()
        assert (tmp_path / "gene-activation_step2_activation.svg").exists()
        assert (tmp_path / "gene-activation_step3_response.svg").exists()
        assert (tmp_path / "gene-activation_static_fallback.svg").exists()
        assert (tmp_path / "gene-activation_animation_manifest.json").exists()
