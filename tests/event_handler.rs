use async_trait::async_trait;
use event_bridge::EventBridge;
use mockall::{mock, predicate::eq};

// Test struct for complex types
#[derive(Debug, PartialEq, Clone)]
pub struct SomeType {
    pub name: String,
    pub age: i32,
}

type ReturnType = Result<(), String>;

// Test event enum
#[derive(EventBridge)]
#[forward_to_trait(TestApiTrait)]
#[trait_returned_type(ReturnType)]
pub enum TestEvent {
    SetIndex(i32),
    SetName(String),
    Initialize,
    SetSomeType(SomeType),
    MultipleArgs(i32, String),
    MultipleArgsTup((i32, String)),
    NamedField { index: i32, name: String },
}

// Test trait that will be implemented
#[async_trait]
pub trait TestApiTrait {
    async fn set_index(&mut self, index: i32) -> Result<(), String>;
    async fn set_name(&mut self, name: String) -> Result<(), String>;
    async fn initialize(&mut self) -> Result<(), String>;
    async fn set_some_type(&mut self, some_type: SomeType) -> Result<(), String>;
    async fn multiple_args(&mut self, index: i32, name: String) -> Result<(), String>;
    async fn multiple_args_tup(&mut self, args: (i32, String)) -> Result<(), String>;
    async fn named_field(&mut self, index: i32, name: String) -> Result<(), String>;
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
        async fn multiple_args(&mut self, index: i32, name: String) -> Result<(), String>;
        async fn multiple_args_tup(&mut self, args: (i32, String)) -> Result<(), String>;
        async fn named_field(&mut self, index: i32, name: String) -> Result<(), String>;
    }
}

// Test event enum
#[derive(EventBridge)]
#[forward_to_trait(TestApiTraitNoReturnType)]
pub enum TestEvent2 {
    SetIndex(i32),
    SetName(String),
    Initialize,
    SetSomeType(SomeType),
}

// Test trait that will be implemented
#[async_trait]
pub trait TestApiTraitNoReturnType {
    async fn set_index(&mut self, index: i32);
    async fn set_name(&mut self, name: String);
    async fn initialize(&mut self);
    async fn set_some_type(&mut self, some_type: SomeType);
}

// Mock implementation for testing
mock! {
    pub TestApiImplNoReturn {}

    #[async_trait]
    impl TestApiTraitNoReturnType for TestApiImplNoReturn {
        async fn set_index(&mut self, index: i32) ;
        async fn set_name(&mut self, name: String);
        async fn initialize(&mut self);
        async fn set_some_type(&mut self, some_type: SomeType) ;
    }
}

#[tokio::test]
async fn test_primitive_arg() {
    let mut mock = MockTestApiImpl::new();
    mock.expect_set_index()
        .with(eq(42))
        .once()
        .returning(|_| Ok(()));

    assert!(TestEvent::SetIndex(42).forward_to(&mut mock).await.is_ok());
}

#[tokio::test]
async fn test_no_arg() {
    let mut mock = MockTestApiImpl::new();
    mock.expect_initialize().once().returning(|| Ok(()));

    assert!(TestEvent::Initialize.forward_to(&mut mock).await.is_ok());
}

#[tokio::test]
async fn test_error_propagation() {
    let mut mock = MockTestApiImpl::new();
    mock.expect_set_name()
        .with(eq("test".to_string()))
        .once()
        .returning(|_| Err("test error".to_string()));

    let result = TestEvent::SetName("test".to_string())
        .forward_to(&mut mock)
        .await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "test error");
}
#[tokio::test]
async fn test_complex_arg() {
    let mut mock = MockTestApiImpl::new();
    let some_type = SomeType {
        name: "test".to_string(),
        age: 30,
    };
    mock.expect_set_some_type()
        .with(eq(some_type.clone()))
        .once()
        .returning(|_| Ok(()));

    assert!(TestEvent::SetSomeType(some_type)
        .forward_to(&mut mock)
        .await
        .is_ok());
}

#[tokio::test]
async fn test_primitive_arg_no_return() {
    let mut mock = MockTestApiImplNoReturn::new();
    mock.expect_set_index()
        .with(eq(42))
        .once()
        .returning(|_| ());

    TestEvent2::SetIndex(42).forward_to(&mut mock).await;
}

#[tokio::test]
async fn test_no_arg_no_return() {
    let mut mock = MockTestApiImplNoReturn::new();
    mock.expect_initialize().once().returning(|| ());

    TestEvent2::Initialize.forward_to(&mut mock).await;
}

#[tokio::test]
async fn test_complex_arg_no_return() {
    let mut mock = MockTestApiImplNoReturn::new();
    let some_type = SomeType {
        name: "test".to_string(),
        age: 30,
    };
    mock.expect_set_some_type()
        .with(eq(some_type.clone()))
        .once()
        .returning(|_| ());

    TestEvent2::SetSomeType(some_type)
        .forward_to(&mut mock)
        .await;
}
#[tokio::test]
async fn test_multiple_args() {
    let mut mock = MockTestApiImpl::new();
    mock.expect_multiple_args()
        .with(eq(42), eq("test".to_string()))
        .once()
        .returning(|_, _| Ok(()));

    assert!(TestEvent::MultipleArgs(42, "test".to_string())
        .forward_to(&mut mock)
        .await
        .is_ok());
}

#[tokio::test]
async fn test_multiple_args_tuple() {
    let mut mock = MockTestApiImpl::new();
    mock.expect_multiple_args_tup()
        .with(eq((42, "test".to_string())))
        .once()
        .returning(|_| Ok(()));

    assert!(TestEvent::MultipleArgsTup((42, "test".to_string()))
        .forward_to(&mut mock)
        .await
        .is_ok());
}

#[tokio::test]
async fn test_named_fields() {
    let mut mock = MockTestApiImpl::new();
    mock.expect_named_field()
        .with(eq(42), eq("test".to_string()))
        .once()
        .returning(|_, _| Ok(()));

    assert!(TestEvent::NamedField {
        index: 42,
        name: "test".to_string()
    }
    .forward_to(&mut mock)
    .await
    .is_ok());
}

#[tokio::test]
async fn test_string_arg_no_return() {
    let mut mock = MockTestApiImplNoReturn::new();
    mock.expect_set_name()
        .with(eq("test".to_string()))
        .once()
        .returning(|_| ());

    TestEvent2::SetName("test".to_string())
        .forward_to(&mut mock)
        .await;
}
