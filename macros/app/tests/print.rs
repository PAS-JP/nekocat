#![cfg(feature = "print")]
use neko_cat_macros::Print;

#[derive(Debug, Print, Default, Clone)]
#[transporter(async fn procedure() {
   println!("{message}");
   let m = message.to_string();
    assert!(
        m.contains("message")
    );
})]
pub struct User {
    id: i32,
    name: String,
    friend: Friend,
}

#[derive(Debug, Print, Default, Clone)]
#[transporter(async fn procedure() {
   println!("{message}");
   let m = message.to_string();
    assert!(
        m.contains("message")
    );
})]
pub struct Friend {
    id: i32,
    name: String,
}

#[tokio::test]
async fn test_print_user() {
    let user = User {
        id: 1,
        name: "name".into(),
        friend: Friend {
            id: 1,
            name: "name".into(),
        },
    };

    user.print_id().message("custom message for id").err().await;
    user.print_name()
        .message("custom message for name")
        .err()
        .await;

    user.print_id().message("info id message").info().await;
    user.print_id()
        .message("success id message")
        .success()
        .await;
    user.print_id()
        .message("warning id message")
        .warning()
        .await;
    user.print_id()
        .message("critical id message")
        .critical()
        .await;
    user.print_id().message("panic id message").panic().await;
    user.print_id()
        .message("rust debug id message")
        .rust()
        .await;

    user.print_name().message("info name message").info().await;
    user.print_name()
        .message("success name message")
        .success()
        .await;
    user.print_name()
        .message("warning name message")
        .warning()
        .await;
    user.print_name()
        .message("critical name message")
        .critical()
        .await;
    user.print_name()
        .message("panic name message")
        .panic()
        .await;
    user.print_name()
        .message("rust debug name message")
        .rust()
        .await;

    user.print().message("custom message for all").err().await;
    user.print().message("custom message for all").info().await;
    user.print()
        .message("custom message for all")
        .success()
        .await;
    user.print()
        .message("custom message for all")
        .warning()
        .await;
    user.print().message("custom message for all").panic().await;
    user.print()
        .message("custom message for all")
        .critical()
        .await;
    user.print().message("custom message for all").rust().await;
}
