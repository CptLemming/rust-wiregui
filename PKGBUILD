# Maintainer: CptLemming
pkgname=wiregui
pkgver=0.1.1
pkgrel=1
pkgdesc="Wireguard network manager"
license=("MIT")
arch=("x86_64")

pkgver() {
    (git describe --long --tags || echo "$pkgver") | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g'
}

build() {
    return 0
}

package() {
    cd ..
    usrdir="$pkgdir/usr"
    mkdir -p $usrdir
    cargo install --no-track --path . --root "$usrdir"
}

