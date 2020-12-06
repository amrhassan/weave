use crate::error::WeaveResult;
use sysinfo::{ProcessorExt, System, SystemExt};

/// Display a ramp indicator of the CPU core usages
pub fn run() -> WeaveResult<()> {
    let system = {
        let mut s = System::new_all();
        s.refresh_cpu();
        s
    };
    let processors = system.get_processors();
    let ramp = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let ramp_step = 100.0 / ramp.len() as f32;
    for processor in processors {
        let ramp_char_i = (processor.get_cpu_usage() / ramp_step)
            .round()
            .min(ramp.len() as f32 - 1.0) as usize;
        let ramp_char = ramp[ramp_char_i];
        print!("{} ", ramp_char);
    }
    println!("");
    Ok(())
}
