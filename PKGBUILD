# Maintainer: Alex Rawson <ajzecrom@gmail.com>
pkgname='pacreason'
pkgver=0.1.0
pkgrel=1
pkgdesc='Remember why you installed a package'
arch=('x86_64')
license=('Apache' 'MIT')
depends=('gcc-libs')
makedepends=('cargo' 'git')
install="${pkgname}.install"
source=("git+https://github.com/zombiepigdragon/${pkgname}.git")
sha256sums=('SKIP')

prepare() {
    cd "${pkgname}"
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "${pkgname}"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "${pkgname}/target/release/${pkgname}"
    install -Dm0755 -t "$pkgdir/usr/bin/" "${pkgname}/target/release/${pkgname}-hook"

    install -Dm0644 -t "$pkgdir/usr/share/libalpm/hooks/" "${pkgname}/${pkgname}-preinstall.hook"
    install -Dm0644 -t "$pkgdir/usr/share/libalpm/hooks/" "${pkgname}/${pkgname}-postinstall.hook"
}
