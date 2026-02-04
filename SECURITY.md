# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x: |

## Reporting a Vulnerability

If you believe you have found a security vulnerability in web-spec, please report it responsibly by:

1. **Do not** open a public issue
2. Email: security@web-spec.dev
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Any known mitigations

We will respond within 48 hours and keep you updated on the fix progress.

## Security Best Practices

When using web-spec:

- Run in isolated environments (containers, VMs)
- Do not expose webhooks without authentication
- Validate feature files before execution
- Use environment variables for sensitive data
- Keep dependencies updated

## Dependencies

web-spec depends on external crates. We monitor for vulnerabilities and update dependencies regularly.
