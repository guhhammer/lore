# rpm

RPM Package Manager. For equivalent commands in other package managers, see <https://wiki.archlinux.org/title/Pacman/Rosetta>.

## Usage
- `rpm -q httpd` : show version of httpd package
- `rpm -qa 'mariadb*'` : list versions of all matching packages
- `rpm -U path/to/package.rpm --force` : forcibly install a package regardless of currently installed versions
- `rpm -qf /etc/postfix/main.cf` : identify owner of a file and show version of the package
- `rpm -ql kernel` : list package-owned files
- `rpm -qp --scripts package.rpm` : show scriptlets from an RPM file
- `rpm -Va 'php-*'` : show changed, missing and/or incorrectly installed files of matching packages
- `rpm -q --changelog package` : display the changelog of a specific package

## Categories
packages, network, files

## Links
- More information: https://rpm-software-management.github.io/rpm/man/rpm.8
- tldr page: https://tldr.inbrowser.app/pages/linux/rpm
- Wikipedia: https://en.wikipedia.org/wiki/RPM_Package_Manager
