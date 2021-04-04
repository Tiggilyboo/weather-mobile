# Maintainer: Simon Willshire <me@simonwillshire.com>
pkgname=weather-mobile
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h' 'aarch64')

build() {
    return 0
}

package() {
    cargo install --root="$pkgdir" weather-mobile
}
