use nih_plug::prelude::*;

use stftlowpass::Freeze;


fn main() {
    nih_export_standalone::<Freeze>();
}
