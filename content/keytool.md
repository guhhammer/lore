# keytool

A certificate management utility included with Java.

## Usage
- `keytool -genkeypair -v -keystore path/to/file.keystore -alias key_name` : create a keystore
- `keytool -storepasswd -keystore path/to/file.keystore` : change a keystore password
- `keytool -keypasswd -alias key_name -keystore path/to/file.keystore` : change a key's password inside a specific keystore

## Categories
files, security

## Links
- More information: https://docs.oracle.com/en/java/javase/25/docs/specs/man/keytool.html
- tldr page: https://tldr.inbrowser.app/pages/common/keytool
