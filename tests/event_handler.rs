
use async_trait::async_trait;
use event_bridge::GenerateEventHandler;
use mockall::{mock, predicate::eq};

// Test struct for complex types
#[derive(Debug, PartialEq)]
pub struct SomeType {
    pub name: String,
    pub age: i32,
}

// Test event enum
#[derive(GenerateEventHandler)]
#[event_handler_trait(TestApiTrait)]
#[event_handler_error(String)]
pub enum TestEvent {
    SetIndex(i32),
    SetName(String),
    Initialize,
    SetSomeType(SomeType),
}

// Test trait that will be implemented
#[async_trait]
pub trait TestApiTrait {
    async fn set_index(&mut self, index: i32) -> Result<(), String>;
    async fn set_name(&mut self, name: String) -> Result<(), String>;
    async fn initialize(&mut self) -> Result<(), String>;
    async fn set_some_type(&mut self, some_type: SomeType) -> Result<(), String>;
}

// Mock implementation for testing
mock! {
    pub TestApiImpl {}

    #[async_trait]
    impl TestApiTrait for TestApiImpl {
        async fn set_index(&mut self, index: i32) -> Result<(), String>;
        async fn set_name(&mut self, name: String) -> Result<(), String>;
        async fn initialize(&mut self) -> Result<(), String>;
        async fn set_some_type(&mut self, some_type: SomeType) -> Result<(), String>;
    }
}

#[tokio::test]
async fn test_set_index() {
    let mut mock = MockTestApiImpl::new();
    mock.expect_set_index()
        .with(eq(42))
        .once()
        .returning(|_| Ok(()));

    assert!(TestEvent::SetIndex(42).event_handler(&mut mock).await.is_ok());
}

#[tokio::test]
async fn test_initialize() {
    let mut mock = MockTestApiImpl::new();
    mock.expect_initialize()
        .once()
        .returning(|| Ok(()));

    assert!(TestEvent::Initialize.event_handler(&mut mock).await.is_ok());
}

#[tokio::test]
async fn test_error_propagation() {
    let mut mock = MockTestApiImpl::new();
    mock.expect_set_name()
        .with(eq("test".to_string()))
        .once()
        .returning(|_| Err("test error".to_string()));

    let result = TestEvent::SetName("test".to_string())
        .event_handler(&mut mock)
        .await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "test error");
}