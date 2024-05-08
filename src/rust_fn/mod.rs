use csv::ReaderBuilder;
use glob::{glob, GlobError};
use indicatif::ProgressBar;
use ndarray::{concatenate, Array2, ArrayView2, Axis, Slice};
use ndarray_csv::Array2Reader;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn read_layers(folder: &str) -> Array2<f64> {
    let glob_string: String = folder.to_owned() + "/*.pcd";
    let mut glob_iterator: Vec<PathBuf> = glob(glob_string.as_str())
        .expect("Files not found!")
        .collect::<Result<Vec<PathBuf>, GlobError>>()
        .unwrap();
    glob_iterator.par_sort_unstable_by(|a, b| get_z(a).partial_cmp(&get_z(b)).unwrap());
    let len: usize = glob_iterator.len();
    let bar = ProgressBar::new(len as u64);
    let mut arrays: Vec<Array2<f64>> = vec![Array2::<f64>::zeros((0, 0)); len];
    let mut z_vals: Vec<f64> = vec![0.; len];
    let mut z_lens: Vec<usize> = vec![0; len];
    glob_iterator
        .par_iter()
        .zip(arrays.par_iter_mut())
        .zip(z_vals.par_iter_mut())
        .zip(z_lens.par_iter_mut())
        .for_each(
            |(((filepath, array_element), z_vals_element), z_lens_element)| {
                let (array, z, z_len) = read_file(filepath.to_path_buf()).unwrap();
                *array_element = array;
                *z_vals_element = z;
                *z_lens_element = z_len;
                bar.inc(1)
            },
        );

    let mut padding_arrays: Vec<Array2<f64>> = Vec::<Array2<f64>>::new();
    for (z, z_len) in z_vals.iter().zip(z_lens) {
        let z_array: Array2<f64> = Array2::from_elem((z_len, 1), *z);
        padding_arrays.push(z_array);
    }

    let padding_array_views: Vec<ArrayView2<f64>> =
        padding_arrays.iter().map(|x| x.view()).collect();
    let array_views: Vec<ArrayView2<f64>> = arrays.iter().map(|x| x.view()).collect();

    let mut out_array = concatenate(
        Axis(1),
        &[
            concatenate(Axis(0), &array_views)
                .unwrap()
                .slice_axis(Axis(1), Slice::from(0..2)),
            concatenate(Axis(0), &padding_array_views).unwrap().view(),
            concatenate(Axis(0), &array_views)
                .unwrap()
                .slice_axis(Axis(1), Slice::from(2..4)),
        ],
    )
    .unwrap();

    out_array.column_mut(0).par_map_inplace(correct_x);
    out_array.column_mut(1).par_map_inplace(correct_y);

    out_array
}

pub fn read_selected_layers(file_list: Vec<PathBuf>) -> Array2<f64> {
    let len: usize = file_list.len();
    let bar = ProgressBar::new(len as u64);
    let mut arrays: Vec<Array2<f64>> = vec![Array2::<f64>::zeros((0, 0)); len];
    let mut z_vals: Vec<f64> = vec![0.; len];
    let mut z_lens: Vec<usize> = vec![0; len];
    file_list
        .par_iter()
        .zip(arrays.par_iter_mut())
        .zip(z_vals.par_iter_mut())
        .zip(z_lens.par_iter_mut())
        .for_each(
            |(((filepath, array_element), z_vals_element), z_lens_element)| {
                let (array, z, z_len) = read_file(filepath.to_path_buf()).unwrap();
                *array_element = array;
                *z_vals_element = z;
                *z_lens_element = z_len;
                bar.inc(1)
            },
        );

    let mut padding_arrays: Vec<Array2<f64>> = Vec::<Array2<f64>>::new();
    for (z, z_len) in z_vals.iter().zip(z_lens) {
        let z_array: Array2<f64> = Array2::from_elem((z_len, 1), *z);
        padding_arrays.push(z_array);
    }

    let padding_array_views: Vec<ArrayView2<f64>> =
        padding_arrays.iter().map(|x| x.view()).collect();
    let array_views: Vec<ArrayView2<f64>> = arrays.iter().map(|x| x.view()).collect();

    let mut out_array = concatenate(
        Axis(1),
        &[
            concatenate(Axis(0), &array_views)
                .unwrap()
                .slice_axis(Axis(1), Slice::from(0..2)),
            concatenate(Axis(0), &padding_array_views).unwrap().view(),
            concatenate(Axis(0), &array_views)
                .unwrap()
                .slice_axis(Axis(1), Slice::from(2..4)),
        ],
    )
    .unwrap();

    out_array.column_mut(0).par_map_inplace(correct_x);
    out_array.column_mut(1).par_map_inplace(correct_y);

    out_array
}

pub fn read_layer(file: &str) -> Array2<f64> {
    let (array, z, z_len) = read_file(Path::new(file).to_path_buf()).unwrap();
    let z_array: Array2<f64> = Array2::from_elem((z_len, 1), z);
    let z_array_view: ArrayView2<f64> = z_array.view();
    let array_view: ArrayView2<f64> = array.view();

    let mut out_array = concatenate(Axis(1), &[array_view, z_array_view]).unwrap();

    out_array.column_mut(0).par_map_inplace(correct_x);
    out_array.column_mut(1).par_map_inplace(correct_y);

    out_array
}

pub fn read_file(filepath: PathBuf) -> Result<(Array2<f64>, f64, usize), Box<dyn Error>> {
    let z: f64 = get_z(&filepath);
    let file = File::open(filepath)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_reader(file);
    let array_read: Array2<f64> = rdr.deserialize_array2_dynamic()?;
    let z_len: usize = array_read.shape()[0];

    Ok((array_read, z, z_len))
}

pub fn get_z(filepath: &PathBuf) -> f64 {
    filepath
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<f64>()
        .unwrap()
}

pub fn correct_x(x: &mut f64) -> () {
    *x = -((((*x + 16384.) * 0.009155273) - 87.) / 1.01);
}

pub fn correct_y(y: &mut f64) -> () {
    *y = (((*y + 16384.) * 0.009155273) - 91.) / 1.02;
}

#[cfg(test)]
mod tests {
    use crate::rust_fn;
    use anyhow::{Error, Result};
    use approx::{AbsDiffEq, UlpsEq};
    use arbitrary::{Arbitrary, Result as ArbResult, Unstructured};
    use arbtest::arbtest;
    use flexbuffers::Reader;
    use float_cmp::{ApproxEq, F64Margin};
    use ndarray::{Array, Array1, Array2, Zip};
    use serde::Deserialize;
    use std::fs::File;
    use std::io::{
        BufWriter, Error as ioError, ErrorKind as ioErrorKind, Read, Result as ioResult, Write,
    };
    use std::path::PathBuf;
    use tar::Archive;
    use tempfile::{tempdir, TempDir};
    use xz::read::XzDecoder;

    const TEST_DATA_FILE: &str = "tests/data.tar.xz";
    const Z_SCALING: f64 = f64::MAX / 100.0;

    #[derive(Debug)]
    struct ExampleData {
        z: f64,
        ar: Array2<isize>,
    }
    impl<'a> Arbitrary<'a> for ExampleData {
        fn arbitrary(u: &mut Unstructured<'a>) -> ArbResult<Self> {
            // NOTE: We need to add 1 here, it len == 0 the test will fail
            let len = 1 + u
                .arbitrary_len::<(isize, isize, isize, isize)>()?
                .min(usize::MAX - 1);
            let ar =
                Array2::from_shape_simple_fn((4, len), || isize::arbitrary(u).unwrap_or_default());
            let z = f64::arbitrary(u)?.abs() / Z_SCALING;

            Ok(ExampleData { z, ar })
        }
    }

    #[derive(Debug)]
    struct ReadResult {
        z: f64,
        ar: Array2<f64>,
    }
    impl From<ExampleData> for ReadResult {
        fn from(value: ExampleData) -> ReadResult {
            ReadResult {
                z: value.z,
                ar: value.ar.mapv(|x| x as f64),
            }
        }
    }
    impl ApproxEq for ReadResult {
        type Margin = F64Margin;

        fn approx_eq<T: Into<F64Margin>>(self, other: Self, margin: T) -> bool {
            let margin = margin.into();

            if !self.z.approx_eq(other.z, margin) {
                return false;
            } else if self.ar.shape() != other.ar.shape() {
                return false;
            } else {
                Zip::from(&self.ar)
                    .and(&other.ar)
                    .fold(true, |acc, &a, &b| acc && a.approx_eq(b, margin))
            }
        }
    }

    fn write_array_to_pcd(file: File, array: &Array2<isize>) -> ioResult<()> {
        let mut filebuf = BufWriter::new(file);

        for row in array.rows().into_iter() {
            let mut row_string = String::new();
            for &item in row {
                row_string.push_str(&format!("{} ", item));
            }
            row_string.pop(); // Remove the last space
            writeln!(filebuf, "{}", row_string)?;
        }

        Ok(())
    }

    fn create_test_pcd(z: f64, ar: &Array2<isize>) -> ioResult<(TempDir, PathBuf)> {
        let tmpd = tempdir()?;
        let tmpfpath = tmpd.path().join(format!("{:.32}.pcd", z));
        let tmpf = File::create(tmpfpath.clone())?;
        write_array_to_pcd(tmpf, ar)?;
        // tmpd needs to be returned or else it gets deleted when it goes out of scope
        Ok((tmpd, tmpfpath))
    }

    #[test]
    fn proptest_read_file() {
        fn prop(u: &mut Unstructured<'_>) -> ArbResult<()> {
            let data = ExampleData::arbitrary(u)?;
            let (_tmpdir, tmpfpath) = create_test_pcd(data.z, &data.ar).unwrap();
            let (ar_out, z_out, _) = rust_fn::read_file(tmpfpath).unwrap();
            let actual_result = ReadResult {
                z: z_out,
                ar: ar_out,
            };
            let example_result: ReadResult = data.into();
            assert!(example_result.approx_eq(actual_result, F64Margin::default()));
            Ok(())
        }
        arbtest(prop);
    }

    fn unpack_test_data(test_name: &str) -> Result<Array1<f64>> {
        let compressed_file = File::open(TEST_DATA_FILE)?;
        let decompressor = XzDecoder::new(compressed_file);
        let mut archive = Archive::new(decompressor);
        let mut buffer = Vec::new();
        for maybe_entry in archive.entries()? {
            let mut entry = maybe_entry?;
            let path = entry.path()?;
            let path_str = path.to_string_lossy();

            if path_str == test_name {
                entry.read_to_end(&mut buffer)?;
                break;
            }
        }

        if buffer.len() > 0 {
            let flexbuf = Reader::get_root(buffer.as_slice())?;
            let ar: Array1<f64> = Array1::deserialize(flexbuf)?;
            Ok(ar)
        } else {
            let err = ioError::new(
                ioErrorKind::NotFound,
                format!("Entry {} not found in {}", test_name, TEST_DATA_FILE),
            );
            Err(Error::new(err))
        }
    }

    #[test]
    fn proptest_correct_x() {
        fn prop(u: &mut Unstructured<'_>) -> ArbResult<()> {
            let mut input = f64::arbitrary(u)?;
            let output = rust_fn::correct_x(&mut input);
            Ok(output)
        }
        arbtest(prop);
    }

    #[test]
    fn regrtest_correct_x() {
        let true_x = unpack_test_data("correct_x_out.flex").unwrap();
        let mut test_x = Array::range(-1000000., 1000000., 1.);
        test_x.par_map_inplace(rust_fn::correct_x);
        assert!(true_x
            .iter()
            .zip(test_x.iter())
            .all(|(&x, y): (&f64, &f64)| -> bool {
                x.ulps_eq(y, f64::default_epsilon(), f64::default_max_ulps())
            }));
    }

    #[test]
    fn proptest_correct_y() {
        fn prop(u: &mut Unstructured<'_>) -> ArbResult<()> {
            let mut input = f64::arbitrary(u)?;
            let output = rust_fn::correct_y(&mut input);
            Ok(output)
        }
        arbtest(prop);
    }

    #[test]
    fn regrtest_correct_y() {
        let true_y = unpack_test_data("correct_y_out.flex").unwrap();
        let mut test_y = Array::range(-1000000., 1000000., 1.);
        test_y.par_map_inplace(rust_fn::correct_y);
        assert!(true_y
            .iter()
            .zip(test_y.iter())
            .all(|(&x, y): (&f64, &f64)| -> bool {
                x.ulps_eq(y, f64::default_epsilon(), f64::default_max_ulps())
            }));
    }
}
