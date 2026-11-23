# yesod

Helper tool for Yesod, a Haskell-based web framework. All Yesod commands are invoked through the `stack` project manager.

## Usage
- `stack new my-project yesod-sqlite` : create a new scaffolded site, with SQLite as backend, in the `my-project` directory
- `stack build yesod-bin cabal-install --install-ghc` : install the Yesod CLI tool within a Yesod scaffolded site
- `stack exec -- yesod devel` : start development server
- `stack exec -- yesod touch` : touch files with altered Template Haskell dependencies
- `stack exec -- yesod keter` : deploy application using Keter (Yesod's deployment manager)

## Categories
development, packages

## Links
- More information: https://github.com/yesodweb/yesod
- tldr page: https://tldr.inbrowser.app/pages/common/yesod
