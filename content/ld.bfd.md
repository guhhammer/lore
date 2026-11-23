# ld.bfd

The GNU linker.

## Usage
- `ld [options] objfile ...` : the general form, as the manual gives it
- `ld.bfd -a keyword` : this option is supported for HP/UX compatibility
- `ld.bfd --audit AUDITLIB` : adds AUDITLIB to the "DT_AUDIT" entry of the dynamic section
- `ld.bfd --format=input-format` : ld may be configured to support more than one kind of object file
- `ld.bfd -P AUDITLIB` : adds AUDITLIB to the "DT_DEPAUDIT" entry of the dynamic section

## Categories
development

## Links
- Documentation: https://systemd.io/ELF_PACKAGE_METADATA/
