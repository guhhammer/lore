# x86_64-linux-gnu-ld

The GNU linker.

## Usage
- `ld [options] objfile ...` : the general form, as the manual gives it
- `x86_64-linux-gnu-ld -a keyword` : this option is supported for HP/UX compatibility
- `x86_64-linux-gnu-ld --audit AUDITLIB` : adds AUDITLIB to the "DT_AUDIT" entry of the dynamic section
- `x86_64-linux-gnu-ld --format=input-format` : ld may be configured to support more than one kind of object file
- `x86_64-linux-gnu-ld -P AUDITLIB` : adds AUDITLIB to the "DT_DEPAUDIT" entry of the dynamic section

## Categories
development

## Links
- Documentation: https://systemd.io/ELF_PACKAGE_METADATA/
