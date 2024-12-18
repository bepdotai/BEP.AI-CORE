//! Google Gemini API client and Bep integration
//!
//! # Example
//! ```
//! use bep::providers::google;
//!
//! let client = google::Client::new("YOUR_API_KEY");
//!
//! let gemini_embedding_model = client.embedding_model(google::EMBEDDING_001);
//! ```

pub mod client;
pub mod completion;
pub mod embedding;
pub use client::Client;

pub mod gemini_api_types {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum ExecutionLanguage {
        /// Unspecified language. This value should not be used.
        LanguageUnspecified,
        /// Python >= 3.10, with numpy and simpy available.
        Python,
    }

    /// Code generated by the model that is meant to be executed, and the result returned to the model.
    /// Only generated when using the CodeExecution tool, in which the code will be automatically executed,
    /// and a corresponding CodeExecutionResult will also be generated.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct ExecutableCode {
        /// Programming language of the code.
        pub language: ExecutionLanguage,
        /// The code to be executed.
        pub code: String,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CodeExecutionResult {
        /// Outcome of the code execution.
        pub outcome: CodeExecutionOutcome,
        /// Contains stdout when code execution is successful, stderr or other description otherwise.
        pub output: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum CodeExecutionOutcome {
        /// Unspecified status. This value should not be used.
        Unspecified,
        /// Code execution completed successfully.
        Ok,
        /// Code execution finished but with a failure. stderr should contain the reason.
        Failed,
        /// Code execution ran for too long, and was cancelled. There may or may not be a partial output present.
        DeadlineExceeded,
    }
}
