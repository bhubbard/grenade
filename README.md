# noslop

A Claude Code plugin (a single skill) that makes or fixes Claude artifacts so anyone understands them in 5 seconds: always HTML, the form picked from the reader's job, visuals instead of walls of text, a header that says what it is.

Forked from [rubenhassid1/grenade](https://github.com/rubenhassid1/grenade) by Ruben Hassid (How to AI), MIT. Changes are listed in [CHANGELOG.md](CHANGELOG.md).

**The rules live in one place: [SKILL.md](plugins/noslop/skills/noslop/SKILL.md).** This README does not repeat them.

## Install

```
/plugin marketplace add bhubbard/grenade
/plugin install noslop@noslop
```

Or point Claude at the file in a chat: `Follow github.com/bhubbard/grenade`.

## Use

Make a page (add your notes or files first). Say "HTML page", or Claude may turn it into a plain doc:

```
Make an HTML page about [YOUR TOPIC] for [WHO WILL READ IT]. Follow github.com/bhubbard/grenade
```

Fix one (add the page to the chat first):

```
Fix this artifact. Follow github.com/bhubbard/grenade
```

Web version (React site on GitHub and Vercel behind a soft passcode; not for confidential data). Open Claude Code in an empty folder, then:

```
Make a website about [YOUR TOPIC] for [WHO WILL READ IT]. Follow github.com/bhubbard/grenade, the web version.
```

## Lint a page

`tools/noslop-lint` is a small Rust CLI that checks an HTML file against the mechanical rules: only the 5 font sizes, no emoji, no `nowrap`, no gradients.

```
cargo run --manifest-path tools/noslop-lint/Cargo.toml -- examples/decision-page.html
```

It exits 1 when it finds a problem.

## Examples

Open the files in [examples/](examples/): decision-page, monthly-report, hiring-dashboard, expense-guide, pitch-slides, deal-pricer, support-cheat-sheet, and the before and after pair sloppy-dashboard and fixed-dashboard. Each is a standalone HTML file.

## Credits

Original by [Ruben Hassid](https://www.linkedin.com/in/ruben-hassid/), from the [How to AI newsletter](https://ruben.substack.com). Thanks to [unslop](https://github.com/heydamir/unslop) by Damir Sokolovsky for three ideas the original used. MIT license.
