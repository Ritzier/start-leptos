use std::future::Future;

use tokio::task::{JoinError, JoinSet};

pub struct TaskSupervisor<E>
where
    E: Send + 'static,
{
    tasks: JoinSet<Result<(), E>>,
}

impl<E> TaskSupervisor<E>
where
    E: Send + std::fmt::Display + 'static,
{
    pub fn new() -> Self {
        Self {
            tasks: JoinSet::new(),
        }
    }

    /// Spawn a background task.
    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = Result<(), E>> + Send + 'static,
        E: Send + 'static,
    {
        self.tasks.spawn(future);
    }

    /// Wait for the next completed task.
    pub async fn join_next(&mut self) -> Option<Result<Result<(), E>, JoinError>> {
        self.tasks.join_next().await
    }

    /// Wait for every remaining task.
    pub async fn drain(mut self) {
        while let Some(result) = self.tasks.join_next().await {
            Self::handle_result(result);
        }
    }

    /// Log a task result.
    pub fn handle_result(result: Result<Result<(), E>, JoinError>) {
        match result {
            Ok(Ok(())) => {
                tracing::info!("Task exited");
            }

            Ok(Err(err)) => {
                tracing::error!("{err}");
            }

            Err(join_err) => {
                tracing::error!("Task panicked: {join_err}");
            }
        }
    }
}

impl<E> Default for TaskSupervisor<E>
where
    E: Send + std::fmt::Display + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
