use bitmasher::pipeline::{process_str_pipeline};
use bitmasher::pipeline_inverse::invert_pipeline;

fn main() {
    let salt = Some(b"bitmasher-salt".as_ref());
    let info = b"bitmasher:pipeline:demo";

    let fwd = process_str_pipeline::<64>("Hello🙂", salt, info);
    let inv = invert_pipeline(&fwd, salt, info);

    println!("Recovered string: {:?}", inv.restored_original);
    println!("HKDF matches: {}", inv.hkdf_matches);
}
