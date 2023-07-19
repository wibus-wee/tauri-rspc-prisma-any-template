# Tauri + Rspc + Prisma + Any Framework Template

## Stacks

- [Tauri](https://tauri.app)
- [Rspc](https://www.rspc.dev)
- [Prisma](https://www.prisma.io)
- [ESlint](https://eslint.org)
  - using `@antfu/eslint-config` as base
- [simple-git-hooks](https://github.com/toplenboren/simple-git-hooks)
- [lint-staged](https://github.com/okonet/lint-staged)

## Where is the frontend?

This template does not bind the frontend. Because I think binding the frontend is equivalent to kidnapping someone. 😂

And Tauri is very simple to configure *(at least simpler than Electron in my opinion)*. So not binding the front end is not a big problem for use.

## Usage

You couldn't launch this template as you clone this project. Because this template does not bind the frontend. 

You need to initialize the front-end and configure all the files that need to be configured before you can start.

1. Click the **Use this template** button.
2. Clone your repo to local space.
3. Run `pnpm i`  to install necessary dependencies.
4. Modify the file according to the "[What files you should edit](#what-files-you-should-edit)" section
5. Run `pnpm generate:prisma` to generate prisma client first.

## What files you should edit

- `core/src/utils.rs`
- `core/Cargo.toml`
- `src-tauri/tauri.conf.json`
- `src-tauri/src/main.rs`
- `prisma/schema.prisma`
- `.github/workflows/test.yml` -- Unblock comments that prevent autorun action.
- `.gitignore` -- Maybe you don’t want to push the build files to the repo

Here are some placeholders you should replace:

- `<your_app_name>` - App name
- `<your_app_desc>` - Description of your app
- `<your_app_identifier>` - App identifier (used for Tauri)
- `<your_app_corename>` - The core of you app.

## Tips

- Each time the application starts, you must call `db.migrate_and_populate`, otherwise the program may not be able to write data.
- Normally, you don't need to modify anything inside `core/prisma`. That's just to start the client. `cargo prisma`
- In `src-tauri/src/main.rs`, I force the `RUST LOG` env to debug.
- `core/prisma.rs` should not be uploaded to the remote repository, but should be generated every time using `pnpm generate:prisma`.

## Utils  `core/src/utils.rs`

- `get_os_info` - Get the OS info: `(os, arch)`
- `get_app_dir` - Get the app dir: `~/.app` on Linux, `%APPDATA%/app` on Windows and `~/.local/share/app` on MacOS

## Structure

- `core`
- `prisma`
- `src-tauri`
- your frontend

## References

\- [twidgeapp/twidge](https://github.com/twidgeapp/twidge) for its structure and code
