use crate::config::app::AppConfiguration;
use crate::std::result::Result;
use crate::templates::export::FileExportTemplate;
use crate::templates::traits::FileTemplate;
use super::options::FileExportOptions;
use super::traits::DataPipeline;


pub(crate) struct FileExportPipeline {
    template: FileExportTemplate,
    config: AppConfiguration
}


impl FileExportPipeline {
    #[allow(dead_code)]
    fn template(&self) -> &FileExportTemplate {
        &self.template
    }


    #[allow(dead_code)]
    fn config(&self) -> &AppConfiguration {
        &self.config
    }
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
        println!("{:?}", FileExportTemplate::list(Vec::<Result<std::path::PathBuf>>::new()));
        Ok(())
    }
}
