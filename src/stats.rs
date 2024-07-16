use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::path::Path;

use image_match::cosine_similarity;
use image_match::image::get_file_signature;

pub fn check_match_percentages() {
    let orig = calc_sigs_for_pic_dir_files("original");
    let cropped = calc_sigs_for_pic_dir_files("cropped");
    let grown = calc_sigs_for_pic_dir_files("grown");
    let shrunk = calc_sigs_for_pic_dir_files("shrunk");

    print_stats("Cropped", evaluate_altered(&orig, &cropped));
    print_stats("Grown", evaluate_altered(&orig, &grown));
    print_stats("Shrunk", evaluate_altered(&orig, &shrunk));
    print_stats("Non-Matching", evaluate_non_matching(&orig));
}

fn calc_sigs_for_pic_dir_files(dir: &str) -> HashMap<String, Vec<i8>> {
    println!("Calculating signatures for {}", dir);
    let pics_root = Path::new("./pics").join(Path::new(dir));
    let names: Vec<OsString> = fs::read_dir(pics_root.clone()).unwrap()
        .filter(|f| f.as_ref().unwrap().file_type().unwrap().is_file())
        .map(|f| f.unwrap().file_name())
        .collect();
    let mut files = HashMap::with_capacity(names.len());
    for name in names {
        let path = pics_root.join(Path::new(&name));
        println!("\t{:?}", &path);
        let signature = get_file_signature(path).unwrap();
        files.insert(name.into_string().unwrap(), signature);
    }

    files
}

fn evaluate_altered(orig: &HashMap<String, Vec<i8>>, altered: &HashMap<String, Vec<i8>>) -> Vec<f64> {
    let mut cosines = Vec::with_capacity(orig.len());
    for (file, signature) in orig {
        let altered_sig = altered.get(file).unwrap();
        cosines.push(cosine_similarity(signature, altered_sig));
    }

    return cosines
}

fn evaluate_non_matching(orig: &HashMap<String, Vec<i8>>) -> Vec<f64> {
    let mut non_matching = Vec::with_capacity(orig.len() * orig.len());
    for (name, signature) in orig {
        orig.iter().filter(|(n, _)| n != &name)
            .map(|(_, other_sig)| cosine_similarity(signature, other_sig))
            .for_each(|similarity| non_matching.push(similarity));
    }

    return non_matching
}

fn print_stats(name: &str, mut cosines: Vec<f64>) {
    cosines.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let avg = cosines.iter().sum::<f64>() / cosines.len() as f64;

    println!("{}:", name);
    println!("Min\tMean\tMax\t30th\t50th\t75th\t90th\t95th\t99th");
    println!("{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\n",
             cosines.first().unwrap(),
             avg,
             cosines.last().unwrap(),
             prcnt(&cosines, 0.30),
             prcnt(&cosines, 0.50),
             prcnt(&cosines, 0.75),
             prcnt(&cosines, 0.90),
             prcnt(&cosines, 0.95),
             prcnt(&cosines, 0.99),
    );
}

fn prcnt(cosines: &Vec<f64>, percentile: f64) -> f64 {
    let idx = (percentile * cosines.len() as f64).floor() as usize;
    *cosines.get(idx).unwrap()
}

