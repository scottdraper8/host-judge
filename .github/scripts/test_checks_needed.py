import unittest

from checks_needed import checks_needed


class ChecksNeededTests(unittest.TestCase):
    def test_known_non_build_files_are_skipped(self):
        self.assertFalse(checks_needed([b"README.md", b".gitignore", b".agents/skill.md"]))

    def test_relevant_and_unknown_files_run_checks(self):
        for path in (b"src/main.rs", b"Cargo.lock", b".github/workflows/ci.yml", b"docs/new.md"):
            with self.subTest(path=path):
                self.assertTrue(checks_needed([b"README.md", path]))

    def test_empty_change_set_fails_safe(self):
        self.assertTrue(checks_needed([]))

    def test_directory_prefix_does_not_match_similar_names(self):
        self.assertTrue(checks_needed([b".agents-malicious/file"]))


if __name__ == "__main__":
    unittest.main()
