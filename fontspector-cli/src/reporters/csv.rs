use super::RunResults;
use crate::{reporters::Reporter, Args};
use fontspector_checkapi::Registry;
use itertools::Itertools;

pub(crate) struct CsvReporter {
    filename: String,
}

impl CsvReporter {
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
        }
    }
}

impl Reporter for CsvReporter {
    fn report(&self, results: &RunResults, args: &Args, _registry: &Registry) {
        #[allow(clippy::unwrap_used)]
        let mut wtr = csv::Writer::from_writer(std::fs::File::create(&self.filename).unwrap());
        #[allow(clippy::unwrap_used)]
        wtr.write_record(["Filename", "Section", "Check ID", "Status", "Codes"])
            .unwrap();

        let organised_results = results.organize();
        for (filename, sectionresults) in organised_results
            .iter()
            .sorted_by_key(|(t, _s)| t.to_string())
        {
            for (section, results) in sectionresults.iter() {
                for result in results.iter() {
                    let subresults = result
                        .subresults
                        .iter()
                        .filter(|c| c.severity >= args.loglevel)
                        .collect::<Vec<_>>();
                    #[allow(clippy::unwrap_used)]
                    wtr.write_record(&[
                        filename.clone(),
                        section.clone(),
                        result.check_id.clone(),
                        result.worst_status().to_string(),
                        subresults
                            .iter()
                            .map(|r| r.code.as_deref().unwrap_or_default())
                            .join(" "),
                    ])
                    .unwrap();
                }
            }
        }
    }
}
