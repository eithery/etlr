use crate::config::app::AppConfiguration;
use crate::std::result::Result;
use crate::templates::export::FileExportTemplate;
use super::options::FileExportOptions;
use super::traits::DataPipeline;


#[allow(dead_code)]
pub(crate) struct FileExportPipeline {
    template: FileExportTemplate,
    config: AppConfiguration
}


impl DataPipeline for FileExportPipeline {
    type Options<'a> = FileExportOptions<'a>;
    type Template = FileExportTemplate;

    const PIPELINE_NAME: &'static str = "Export pipeline";


    fn new(template: FileExportTemplate, config: AppConfiguration) -> Self {
        Self {
            template,
            config
        }
    }


    fn run<'a>(&self, _options: FileExportOptions<'a>) -> Result<()> {
        // let rt = Arc::new(Runtime::new()?);
        // let connection_string = "mssql://sa:mandozebra_9@localhost:1433/packageme";
        // let query = "select * from dtcc.networking_b5051";
        // let source = MsSQLSource::new(rt, connection_string, 1)?;
        // let mut destination = ArrowDestination::new();
        // let dispatcher = Dispatcher::<MsSQLSource, ArrowDestination, MsSQLArrowTransport>::new(
        //     source,
        //     &mut destination,
        //     &[query],
        //     None
        // );
        // dispatcher.run()?;
        // let df = destination.polars()?;
        println!("{:?}", "Test");
        Ok(())
    }
}
