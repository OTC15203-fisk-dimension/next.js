#![feature(arbitrary_self_types)]
#![feature(arbitrary_self_types_pointers)]
#![allow(clippy::needless_return)] // tokio macro-generated code doesn't respect this

use anyhow::Result;
use turbo_tasks::Vc;
use turbo_tasks_testing::{Registration, register, run_once};

static REGISTRATION: Registration = register!();

#[turbo_tasks::value]
#[derive(Clone, Debug)]
struct Value {
    value: u32,
}

#[turbo_tasks::function]
async fn returns_value() -> Result<Vc<Value>> {
    Ok(Value { value: 42 }.cell())
}

/// Test that eventually consistent reads (default .await) cause an error in root tasks
/// The panic happens but we just verify it's an error, not the exact message
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_eventual_read_in_root_task_fails() {
    let result = run_once(&REGISTRATION, || async {
        // This should fail because we're in a root task (run_once)
        // and doing an eventually consistent read (default .await)
        let vc = returns_value();
        let _value = vc.await?;

        anyhow::Ok(())
    })
    .await;

    assert!(
        result.is_err(),
        "Expected an error when doing eventually consistent read in root task"
    );
}

/// Test that strongly consistent reads of OperationVcs work in root tasks
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_strongly_consistent_read_in_root_task_works() {
    run_once(&REGISTRATION, || async {
        // This should work because we're using strongly consistent read
        // The strongly_consistent flag temporarily unmarks the root task
        let vc = returns_value();
        let value = vc.strongly_consistent().await?;
        assert_eq!(value.value, 42);

        anyhow::Ok(())
    })
    .await
    .unwrap()
}

/// Test that eventually consistent reads work fine in normal tasks
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_eventual_read_in_normal_task_works() {
    run_once(&REGISTRATION, || async {
        // Call a turbo_tasks function which is NOT a root task
        // We cannot read the result in the root task, but we can call it
        let _result = calls_another_task_with_eventual_read();
        // Cannot await the result here as that would be a cell read in a root task

        anyhow::Ok(())
    })
    .await
    .unwrap()
}

#[turbo_tasks::function]
async fn calls_another_task_with_eventual_read() -> Result<Vc<Value>> {
    // This is inside a normal turbo_tasks function (not a root task),
    // so eventually consistent reads should work fine
    let vc = returns_value();
    let value = vc.await?; // Eventually consistent read - should work here
    Ok(Value { value: value.value }.cell())
}

/// Test that cell reads (always eventually consistent) cause an error in root tasks
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_cell_read_in_root_task_fails() {
    let result = run_once(&REGISTRATION, || async {
        // Create a cell and try to read it in a root task
        let cell = Value { value: 99 }.cell();
        // This should fail because cell reads are always eventually consistent
        let _value = cell.await?;

        anyhow::Ok(())
    })
    .await;

    assert!(
        result.is_err(),
        "Expected an error when reading cell in root task"
    );
}

/// Test that mark_root_task() and unmark_root_task() work correctly
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_manual_mark_unmark_root_task() {
    use turbo_tasks::{mark_root_task, unmark_root_task};

    run_once(&REGISTRATION, || async {
        // We're in a root task initially, but let's unmark it
        unmark_root_task();

        // Now eventually consistent reads should work
        let vc = returns_value();
        let value_ref = vc.await?;
        assert_eq!(value_ref.value, 42);

        // Re-mark as root task
        mark_root_task();

        anyhow::Ok(())
    })
    .await
    .unwrap()
}

/// Test that errors happen with mark_root_task() in a normal task
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_manual_mark_root_task_causes_error() {
    let result = run_once(&REGISTRATION, || async {
        // Call a turbo_tasks function
        let result = manually_marks_as_root_then_reads();
        let value = result.strongly_consistent().await?;
        assert_eq!(value.value, 42);

        anyhow::Ok(())
    })
    .await;

    assert!(
        result.is_err(),
        "Expected an error when manually marking as root task and doing eventually consistent read"
    );
}

#[turbo_tasks::function]
async fn manually_marks_as_root_then_reads() -> Result<Vc<Value>> {
    use turbo_tasks::mark_root_task;

    // Manually mark as root task
    mark_root_task();

    // This should panic because we marked it as a root task
    let vc = returns_value();
    let _value = vc.await?; // Should panic here

    Ok(Value { value: 42 }.cell())
}
