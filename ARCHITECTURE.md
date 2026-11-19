# Architecture

## Goal

Let people in Costa Rica fact check the government and report wrongdoing without fear. The main property: **a person can never be linked across the two modules**, and a whistleblower can never be identified from what the server stores.

## Modules

| | Fact check | Whistleblow |
| --- | --- | --- |
| Frontend | `apps/factcheck-web` | `apps/leaks-web` |
| API | `services/factcheck-api` | `services/leaks-api` |
| Database | own Postgres | own Postgres |
| zk scope | `cr-factcheck-v1` | `cr-leaks-v1` |
| Public? | yes, verdicts are public | no, reports go to vetted journalists |

The two modules share code (`crates/zk`, `crates/api`) but no runtime state. They have separate domains, databases, cookies and sessions.

## Identity: scoped nullifiers

1. The user proves "Costa Rican citizen, 18+" with zkPassport (World ID as an alternative).
2. The proof is bound to the module's scope. The server gets a **nullifier** derived from the person and the scope.
3. The server stores only the nullifier. No name, no document number, no birth date.

Same person + same scope gives the same nullifier, so one person gets one account per module (sybil resistance). Same person + different scope gives an unrelated nullifier, so the modules cannot be joined. `crates/zk` has tests for both properties.

Today `crates/zk` uses a SHA-256 stub in place of the real proof. Replacing it with the zkPassport SDK must keep the same `verify(scope, proof) -> Nullifier` interface.

## Privacy rules

These are hard rules. A change that breaks one is a bug.

- No IP logging. Handlers never read `CF-Connecting-IP`, `X-Forwarded-For` or the peer address.
- No analytics, no third-party scripts, no external fonts or CDNs. The CSP is `default-src 'self'`.
- The modules never share a database, cookie, session or user ID.
- Files are cleaned and encrypted on the client. The server only stores ciphertext.

## Whistleblow upload pipeline (planned)

```
browser: pick file
  -> strip metadata (EXIF/GPS, document properties)
  -> re-render to pixels (Dangerzone style) for images and PDFs
  -> pad to a fixed size bucket
  -> encrypt with the newsroom public key (libsodium sealed box)
  -> upload ciphertext
server: store blob in MinIO, publish to reviewers after a random delay
```

The whistleblow frontend should also have a Tor onion address.

## Fact check integrity (planned)

Each published verdict is hashed and anchored on Starknet. Nobody, including the operators, can quietly edit or delete a verdict later.

## Roadmap

1. **Skeleton** (this commit): workspace, stub verifier with scope tests, both APIs, both frontends, compose, CI.
2. **Real identity:** zkPassport SDK in `crates/zk`, nullifiers in Postgres, privacy tests in CI (no IP logs, no external origins).
3. **Upload pipeline:** client-side sanitize, encrypt, upload. Reviewer inbox.
4. **Fact check flow:** claims, sources, votes, verdicts, Starknet anchoring.
5. **Hardening:** Tor, threat model review, UX polish.

## Open questions

- Hosting. A server tied to one person is a single point of pressure. Consider an established newsroom that already runs SecureDrop for the whistleblow side.
- Who reviews reports, and how they get the decryption keys.
