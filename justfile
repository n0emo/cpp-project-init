examples_dir := "examples"
test_dir := "out"

test name:
    cargo run --quiet -- --force -f {{ examples_dir / name }}.toml new {{ test_dir / name }}
    cmake -S {{ test_dir / name }} -B {{ test_dir / name / "build" }}
    cmake --build {{ test_dir / name / "build" }}
    ctest -T Test --test-dir {{ test_dir / name / "build" }}

@test-all:
    just test minimal
    just test raylib
    just test gtest
