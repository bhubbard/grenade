# Changelog

## 1.6.0
- Forked from rubenhassid1/grenade v1.5.0.
- Plugin and marketplace metadata now point to bhubbard/grenade.
- README shortened to install, use and links. `SKILL.md` is the single source of the rules.
- Web version: the passcode gate is documented as a soft gate; confidential pages use Deployment Protection or Cloudflare Access.
- New section 9 in `SKILL.md`: regulated and bilingual pages (disclosures, no unverifiable claims, language parity, compliance flag).
- New `tools/noslop-lint` Rust CLI for the mechanical rules.
- Examples: removed the ~42 KB Claude host runtime script pasted into each file (124 KB total, down from 497 KB). Pages render the same.
- Style kit unchanged.
