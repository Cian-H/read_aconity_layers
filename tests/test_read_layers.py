from concurrent.futures import ThreadPoolExecutor
import numpy as np
from pathlib import Path
import pytest
import subprocess


TEST_ARRAY = np.arange(
    np.iinfo(np.int32).min,
    np.iinfo(np.int32).max,
    256,  # Limited to ~256MB uncompressed for practicality
).reshape((-1, 4))
N_FILES = 1024
TEST_ZVALS = np.arange(0, 10_240, 10_240 // (N_FILES - 1))


def build_module():
    subprocess.run(["maturin", "develop"])


build_fixture = pytest.fixture(scope="module")(build_module)


def write_layers_to_dir(dir_path):
    output_layers = TEST_ARRAY.reshape((1024, -1, 4))

    def write_layer(ar, z):
        np.savetxt(dir_path / f"{z}.pcd", ar, delimiter=" ", newline="\n", fmt="%i")

    with ThreadPoolExecutor() as p:
        p.map(write_layer, output_layers, TEST_ZVALS)


# Sorts arrays without mixing datapoints
# This is needed because we dont need to guarantee read order
def sort_result(ar):
    ar = ar[ar[:, 4].argsort()]
    ar = ar[ar[:, 3].argsort()]
    ar = ar[ar[:, 0].argsort()]
    ar = ar[ar[:, 1].argsort()]
    ar = ar[ar[:, 2].argsort()]
    return ar


def regenerate_regr_outputs():
    from loguru import logger
    import tempfile

    build_module()
    from read_aconity_layers import read_layers

    dir_path = tempfile.mkdtemp()
    logger.info("Writing temporary layer file...")
    write_layers_to_dir(Path(dir_path))
    logger.info("Reading temporary layer file...")
    output = read_layers(dir_path)
    output_file_path = Path("tests/read_layers_regr.npz")
    logger.info("Sorting outputs...")
    output_sorted = sort_result(output)
    logger.info("Saving outputs to compressed file...")
    np.savez_compressed(output_file_path, output=output_sorted)
    logger.info("Outputs regenerated!")


@pytest.fixture(scope="module")
def shared_dir(tmpdir_factory):
    dir_path = tmpdir_factory.mktemp("read_layers_shared")
    write_layers_to_dir(dir_path)
    return dir_path


@pytest.fixture(scope="module")
def ground_truth():
    return np.load("tests/read_layers_regr.npz")["output"]


def test_read_layers(build_fixture, shared_dir, ground_truth):
    from read_aconity_layers import read_layers

    result = sort_result(read_layers(str(shared_dir)))
    assert np.all(np.isclose(np.argsort(result, 0), np.argsort(ground_truth, 0)))


def test_read_selected_layers(build_fixture, shared_dir, ground_truth):
    from read_aconity_layers import read_selected_layers

    result = sort_result(read_selected_layers([str(x) for x in shared_dir.listdir()]))
    assert np.all(np.isclose(result, ground_truth))


def test_read_layer(build_fixture, shared_dir, ground_truth):
    from read_aconity_layers import read_layer

    x = np.concat([read_layer(str(x)) for x in shared_dir.listdir()], axis=0)
    result = sort_result(x)
    assert np.all(np.isclose(result, ground_truth))


if __name__ == "__main__":
    regenerate_regr_outputs()
