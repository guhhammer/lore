# rdpsign

A tool for signing Remote Desktop Protocol (RDP) files.

## Usage
- `rdpsign path\to\file.rdp` : sign an RDP file
- `rdpsign path\to\file.rdp /sha265 hash` : sign an RDP file using a specific sha256 hash
- `rdpsign path\to\file.rdp /q` : enable quiet output
- `rdpsign path\to\file.rdp /v` : display verbose warnings, messages, and statuses
- `rdpsign path\to\file.rdp /l` : test the signing by displaying the output to `stdout` without updating the file

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, desktop, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/rdpsign
