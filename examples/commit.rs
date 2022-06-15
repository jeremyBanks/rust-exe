#!/usr/bin/env rust
use ::sentry; // to enable tokio rt

fn main() -> ::eyre::Result<()> {
    ::env_logger::init();

    async fn main() -> ::eyre::Result<()> {
        let key = ::jsonwebtoken::EncodingKey::from_rsa_pem(todo!())?;
        let github = ::octocrab::OctocrabBuilder::new()
            .app(210642.into(), key)
            .build()?.installation(26513464.into());

        let result = github.repos("jeremyBanks", "rust-exe").create_file("test12534.txt", "creating test", "content").branch("staging").send()
        .await?;

        dbg!(result);

     Ok(())
    }

    ::tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(main())

}
