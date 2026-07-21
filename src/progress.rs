use crate::global_definition::{ProgressLogShape, ProgressType};


pub struct ProgressTask {
    task: &'static str,
    step: u32,
    total: u32,
}

impl ProgressTask {
    pub fn start(
        task: &'static str,
        total: u32,
        message: impl Into<String>,
    ) -> Self {
        let progress = Self {
            task,
            step: 0,
            total,
        };

        progress.emit(
            ProgressType::Started,
            message.into(),
            None,
        );

        progress
    }

    pub fn step(
        &mut self,
        message: impl Into<String>,
    ) {
        self.step += 1;

        self.emit(
            ProgressType::Step,
            message.into(),
            None,
        );
    }

    pub fn step_with(
        &mut self,
        message: impl Into<String>,
        detail: impl Into<String>,
    ) {
        self.step += 1;

        self.emit(
            ProgressType::Step,
            message.into(),
            Some(detail.into()),
        );
    }

    pub fn finish(
        &self,
        message: impl Into<String>,
    ) {
        self.emit(
            ProgressType::Finished,
            message.into(),
            None,
        );
    }
     pub fn Complete(
        &self,
        message: impl Into<String>,
    ) {
        self.emit(
            ProgressType::Finished,
            message.into(),
            None,
        );
    }


    pub fn fail(
        &self,
        message: impl Into<String>,
    ) {
        self.emit(
            ProgressType::Failed,
            message.into(),
            None,
        );
    }

    fn emit(
        &self,
        stage: ProgressType,
        message: String,
        detail: Option<String>,
        
    ) {
        let event = ProgressLogShape {
            task: self.task.to_string(),
            stage,
            message,
            detail,
            step: self.step,
            total: self.total,
        };

        println!(
            "{}",
            serde_json::to_string(&event)
                .expect("progress event should serialize")
        );
    }
}