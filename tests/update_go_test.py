"""Verify generated source updates preserve handwritten module files."""

from pathlib import Path
import tempfile
import unittest
from tools.update_go import update


class UpdateGoTest(unittest.TestCase):
    def test_update_check_and_removed_definitions(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            generated, destination = root / "generated", root / "go"
            generated.mkdir()
            destination.mkdir()
            (generated / "new.pb.go").write_text("generated")
            (destination / "old.pb.go").write_text("old")
            (destination / "go.mod").write_text("handwritten")
            with self.assertRaisesRegex(ValueError, "Stale Go"):
                update(generated, destination, check=True)
            update(generated, destination)
            update(generated, destination, check=True)
            self.assertFalse((destination / "old.pb.go").exists())
            self.assertEqual((destination / "go.mod").read_text(), "handwritten")
            (destination / "new.pb.go").write_text("changed")
            with self.assertRaisesRegex(ValueError, "Stale Go"):
                update(generated, destination, check=True)


if __name__ == "__main__":
    unittest.main()
