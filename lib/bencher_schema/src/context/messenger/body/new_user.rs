use slog::Logger;
use url::Url;

#[cfg(feature = "plus")]
use crate::context::messenger::GITHUB_OAUTH2;

use super::FmtBody;

#[derive(Debug)]
pub struct NewUserBody {
    pub admin: String,
    pub console_url: Url,
    pub name: String,
    pub email: String,
    pub invited: bool,
    pub method: String,
}

impl FmtBody for NewUserBody {
    fn text(&self) -> String {
        let Self {
            admin,
            console_url,
            name,
            email,
            invited,
            method,
        } = self;
        format!(
            "Ahoy {admin},
        A new user {invited_or_joined} your Bencher instance ({console_url}) via {method}!

        Name: {name}
        Email: {email}

        🐰 Bencher
        ",
            invited_or_joined = invited_or_joined(*invited)
        )
    }

    fn html(&self, _log: &Logger) -> String {
        let Self {
            admin,
            console_url,
            name,
            email,
            invited,
            method,
        } = self;
        #[cfg(feature = "plus")]
        let github_link = (method == GITHUB_OAUTH2)
            .then_some(format!(
                r#"<a href="https://github.com/{name}">View {name} on GitHub</a><br/>"#
            ))
            .unwrap_or_default();
        #[cfg(not(feature = "plus"))]
        let github_link = String::new();
        format!(
            "<!doctype html>
<html>
    <head>
        <meta charset=\"utf-8\" />
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1, shrink-to-fit=no\" />
        <meta name=\"theme-color\" content=\"#ffffff\" />
        <title>New user {invited_or_joined} Bencher</title>
    </head>
    <body>
        <p>Ahoy {admin},</p>
        <p>A new user {invited_or_joined} your Bencher instance ({console_url}) via {method}!</p>
        <br />
        <p>Name: {name}</p>
        <p>Email: {email}</p>
        <br />
        {github_link}
        <br />
        <a href=\"mailto:{email}?subject=Any%20advice%3F\">Ask for advice</a>
        <br />
        <p>🐰 Bencher</p>
    </body>
</html>",
            invited_or_joined = invited_or_joined(*invited)
        )
    }
}

fn invited_or_joined(invited: bool) -> &'static str {
    if invited {
        "was invited to"
    } else {
        "has joined"
    }
}
