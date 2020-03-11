use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use midas_rs::{default, Float, Int, MidasIterator, MidasParams, MidasRParams};

use structopt::StructOpt;

const DELIM: char = ',';

fn load_data<P>(
    input: P,
    is_directed: bool,
) -> Result<Box<dyn Iterator<Item = (Int, Int, Int)>>, String>
where
    P: AsRef<Path>,
{
    let file = File::open(input.as_ref()).map_err(|err| format!("{}", err))?;
    let iter = BufReader::new(file).lines().map(move |line| {
        match line
            .as_ref()
            .unwrap()
            .split(DELIM)
            .map(|field| field.parse().unwrap())
            .collect::<Vec<Int>>()
            .as_slice()
        {
            &[source, destination, time] => (source, destination, time),
            _ => panic!("invalid line: {:?}", line.unwrap()),
        }
    });

    Ok(if is_directed {
        Box::new(iter)
    } else {
        Box::new(iter.flat_map(|(source, destination, time)| {
            vec![(source, destination, time), (destination, source, time)]
        }))
    })
}

#[derive(Debug, StructOpt)]
struct Opts {
    input: PathBuf,

    #[structopt(long)]
    rows: Option<Int>,
    #[structopt(long)]
    buckets: Option<Int>,

    #[structopt(long)]
    directed: bool,
    #[structopt(long)]
    no_relations: bool,

    #[structopt(long)]
    alpha: Option<Float>,
    #[structopt(long)]
    m_value: Option<Int>,
}

fn main() {
    let Opts {
        input,
        rows,
        buckets,
        directed,
        no_relations,
        alpha,
        m_value,
    } = Opts::from_args();

    let rows = rows.unwrap_or(default::NUM_ROWS);
    let buckets = buckets.unwrap_or(default::NUM_BUCKETS);
    let alpha = alpha.unwrap_or(default::ALPHA);
    let m_value = m_value.unwrap_or(default::M_VALUE);

    let data = load_data(&input, directed).unwrap();

    let scores = if no_relations {
        data.midas(MidasParams {
            rows,
            buckets,
            m_value,
        })
    } else {
        data.midas_r(MidasRParams {
            rows,
            buckets,
            m_value,
            alpha,
        })
    };

    use std::io::Write;
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    for score in scores {
        stdout
            .write_all(format!("{:.6}\n", score).as_bytes())
            .unwrap();
    }
}
