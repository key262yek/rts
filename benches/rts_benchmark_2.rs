// // 여러 개념이 추가된 상태에서도 빠른가?

use rts::prelude::*;
use rts::system_mod::cont_circ::{self, ContCircSystem, ContCircSystemArguments};
use rts::target_mod::cont_bulk::{self, ContBulkTarget, ContBulkTargetArguments};
use rts::searcher_mod::{Passive, cont_passive_indep::{self, ContPassiveIndepSearcher, ContPassiveIndepSearcherArguments}};
use criterion::{criterion_group, criterion_main, Criterion};

#[allow(dead_code)]
pub struct Simulation{
    dt : f64,
    num_ensemble : usize,
    idx_set : usize,
    seed : u128,
    output_dir : String,
}

impl_argument_trait!(Simulation, "Simulation", SimulationArguments, 5;
    dt, f64, "Dimensionless Time Step Size",
    num_ensemble, usize, "Number of Ensemble",
    idx_set, usize, "Index of Ensemble Set",
    seed, u128, "Initial Seed for Random Number Generator",
    output_dir, String, "Directory containing output file");

impl SimulationArguments{
    pub fn convert(&self) -> Simulation{
        Simulation{
            dt              : self.dt,
            num_ensemble    : self.num_ensemble,
            idx_set         : self.idx_set,
            seed            : self.seed,
            output_dir      : self.output_dir.clone(),
        }
    }
}

fn rts_arguments() -> Result<(), Error>{
    // 지금 구조에서도 200 ensemble 442ms
    // bottleneck 없이 잘 구성한 듯.

    let args: [&str; 12] = ["10", "2", "0:0", "1", "1", "Uniform", "10000", "1e-3", "1", "1", "12314", "tests/images/"];
    let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    const TOTAL_NUM_ARGS : usize = Simulation::NUM_ARGS + cont_circ::ContCircSystem::NUM_ARGS
                                    + cont_bulk::ContBulkTarget::NUM_ARGS
                                    + cont_passive_indep::ContPassiveIndepSearcher::NUM_ARGS;
    const WIDTH : usize = 15;
    const NUM_SKIP : usize = 0;

    if args.len() - NUM_SKIP != TOTAL_NUM_ARGS{
        eprint!("{} arguments given.\nGiven Arguments : ", args.len() - NUM_SKIP);
        for x in args.iter().skip(NUM_SKIP){
            eprint!("{}  ", x);
        }
        eprintln!("\nYou should give {} arguments like below", TOTAL_NUM_ARGS);
        eprintln!("======================= OVERVIEW OF ARGUMENTS ==========================");
        eprintln!("{}", ContCircSystem::brief_info());
        eprintln!("{}", ContBulkTarget::brief_info());
        eprintln!("{}", ContPassiveIndepSearcher::brief_info());
        eprintln!("{}", Simulation::brief_info());
        eprintln!("========================    DESCRIPTIONS     ==========================");
        eprint!("{}", ContCircSystem::info(WIDTH));
        eprint!("{}", ContBulkTarget::info(WIDTH));
        eprint!("{}", ContPassiveIndepSearcher::info(WIDTH));
        eprint!("{}", Simulation::info(WIDTH));
        return Err(Error::make_error_syntax(ErrorCode::InvalidNumberOfArguments));
    }

    // Arguments parsing
    let idx = NUM_SKIP;

    // System arguments
    let sys_args        : Vec<String>               = args[idx..idx+cont_circ::ContCircSystem::NUM_ARGS].to_vec();
    let sys_args        : ContCircSystemArguments   = ContCircSystem::read_args_from_vec(&sys_args)?;
    let _sys_type       : SystemType                = sys_args.sys_type;
    let sys_size        : f64                       = sys_args.sys_size;
    let dim             : usize                     = sys_args.dim;

    let idx = idx + cont_circ::ContCircSystem::NUM_ARGS;

    // Target arguments
    let target_args : Vec<String>               = args[idx..idx+cont_bulk::ContBulkTarget::NUM_ARGS].to_vec();
    let target_args : ContBulkTargetArguments   = ContBulkTarget::read_args_from_vec(&target_args)?;
    let _target_type: TargetType                = target_args.target_type;
    let target_pos  : Position<f64>             = target_args.target_pos.clone();
    let target_size : f64                       = target_args.target_size;

    let idx = idx + cont_bulk::ContBulkTarget::NUM_ARGS;

    // Searcher arguments
    let searcher_args   : Vec<String>                       = args[idx..idx+cont_passive_indep::ContPassiveIndepSearcher::NUM_ARGS].to_vec();
    let searcher_args   : ContPassiveIndepSearcherArguments = ContPassiveIndepSearcher::read_args_from_vec(&searcher_args)?;
    let _searcher_type  : SearcherType                      = searcher_args.searcher_type;
    let mtype           : MoveType                          = searcher_args.mtype.clone();
    let itype           : InitType<f64>                     = searcher_args.itype.clone();
    let num_searcher    : usize                             = searcher_args.num_searcher;


    let idx = idx + cont_passive_indep::ContPassiveIndepSearcher::NUM_ARGS;

    // Simulation arguments
    let simulation_args : Vec<String>           = args[idx..].to_vec();
    let simulation_args : SimulationArguments   = Simulation::read_args_from_vec(&simulation_args)?;
    let dt              : f64                   = simulation_args.dt;
    let num_ensemble    : usize                 = simulation_args.num_ensemble;
    let idx_set         : usize                 = simulation_args.idx_set;
    let seed            : u128                  = simulation_args.seed;
    let _output_dir      : String                = simulation_args.output_dir.clone();

    // Hash seed and generate random number generator
    let seed : u128 = seed + (628_398_227f64 * sys_size + 431_710_567f64 * dim as f64 + 277_627_711f64 * target_size
                        + 719_236_607f64 * num_searcher as f64 + 570_914_867f64 * idx_set as f64).floor() as u128;
    let mut rng : Pcg64 = rng_seed(seed);

    // Create output directory, file
    // fs::create_dir_all(&output_dir).map_err(Error::make_error_io)?;
    // let filename : String = format!("{}", format_args!("RTS_N_PTL_INDEP_SEARCHER_SYS_SIZE_{}_DIM_{}_TARGET_SIZE_{}_NUMBER_OF_SEARCHER_{}_SET_{}.dat", sys_size, dim, target_size, num_searcher, idx_set));
    // let output = fs::File::create(format!("{}/{}", output_dir, filename)).map_err(Error::make_error_io)?;
    // let mut writer = BufWriter::new(&output);

    // System initiation
    let sys         = ContCircSystem::new(sys_size, dim);                               // System
    let target      = ContBulkTarget::new(target_pos, target_size);                     // Target
    let _simulation  = simulation_args.convert();                                        // Simulation
    let mut single_move : Position<f64>  = sys.position_out_of_system();                // reference of displacement
    match itype{
        InitType::<f64>::SpecificPosition(_pos) => {return Err(Error::make_error_syntax(ErrorCode::InvalidConfiguration))},
        InitType::<f64>::Uniform => (),
    };
    let _searcher    = ContPassiveIndepSearcher::new_uniform(&sys, &target, &mut rng, mtype)?;  // Searcher

    // Store arguments
    // write!(&mut writer, "========================    DESCRIPTIONS    ==========================\n")
    //         .map_err(Error::make_error_io)?;
    // write!(&mut writer, "{}", sys.print_configuration(WIDTH)).map_err(Error::make_error_io)?;
    // write!(&mut writer, "{}", target.print_configuration(WIDTH)).map_err(Error::make_error_io)?;
    // write!(&mut writer, "{}", searcher.print_configuration(WIDTH)).map_err(Error::make_error_io)?;
    // write!(&mut writer, "{}", simulation.print_configuration(WIDTH)).map_err(Error::make_error_io)?;
    // write!(&mut writer, "{}", "========================     DATA STARTS    ==========================\n")
    //         .map_err(Error::make_error_io)?;
    // writer.flush().map_err(Error::make_error_io)?;



    for _i in 0..num_ensemble{
        let mut fpt : f64 = std::f64::MAX;  // First Passage Time
        for _j in 0..num_searcher{          // Ordered statistic
            let mut time : f64 = 0f64;      // Time to find target of single ptl
            let mut searcher = ContPassiveIndepSearcher::new_uniform(&sys, &target, &mut rng, mtype)?;

            while !target.check_find(&searcher.pos)? && time < fpt{
                single_move.clear();                                                    // Clear Displacement
                searcher.random_move_to_vec(&mut rng, dt, &mut single_move)?;               // Get random walk
                sys.check_bc(&mut searcher.pos, &mut single_move)?;                      // Check bc and move
                time += dt;                                                              // Time flows
            }

            if time < fpt{
                fpt = time;
            }
        }

        // Export FPT data
        // write!(&mut writer, "{0:.5e}\n", fpt).map_err(Error::make_error_io)?;
        // writer.flush().map_err(Error::make_error_io)?;
    }
    return Ok(());
}

fn bench_rts(c : &mut Criterion){
    c.bench_function("rts_after_arguments", |b|  b.iter(|| rts_arguments()));
}

criterion_group!(benches, bench_rts);
criterion_main!(benches);
