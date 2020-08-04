# Maintainer: Owen D'Aprile <dev@owen.sh>
pkgname="optdeps"
pkgver=0.1.0
pkgrel=1
pkgdesc="Find optional dependencies for an Arch Linux installation"
url="https://github.com/owendaprile/optdeps/"
license=("GPL3")
arch=("x86_64")
makedeps=("")
options=("strip")
source=("git+https://github.com/owendaprile/optdeps/")
sha1sums=("SKIP")

build() {
    cd optdeps
    cargo build --release --locked --all-features --target-dir=target
}

package() {
    cd optdeps
    install -Dm 755 target/release/${pkgname} -t "${pkgdir}/usr/bin"
}
