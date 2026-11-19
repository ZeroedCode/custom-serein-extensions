use serein_extension_sdk::{AppOutput, HostEffect, MenuInvocation, MenuTarget};

fn handle(input: MenuInvocation) -> AppOutput {
    let Some(MenuTarget::User { user_id, .. }) = input.menu_target else {
        return AppOutput::default();
    };

    let mut out = AppOutput::default();
    out.effects.push(HostEffect::CopyText {
        text: format!("<https://discord.com/users/{user_id}>"),
    });
    out
}

serein_extension_sdk::export!(handle);
